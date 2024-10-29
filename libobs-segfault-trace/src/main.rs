//! A module for handling crashes and writing minidump files
//!
//! Mostly copied from <https://github.com/EmbarkStudios/crash-handling/blob/main/minidumper/examples/diskwrite.rs>
//!
//! TODO: Capture crash dumps on panic. <https://github.com/firezone/firezone/issues/3520>
//!
//! To get human-usable stack traces out of a dump, do this:
//! (Copied from <https://github.com/firezone/firezone/issues/3111#issuecomment-1887975171>)
//!
//! - Get the pdb corresponding to the client exe
//! - `cargo install --locked dump_syms minidump-stackwalk`
//! - Use dump_syms to convert the pdb to a syms file
//! - `minidump-stackwalk --symbols-path firezone.syms crash.dmp`

use anyhow::{anyhow, bail, Context, Result};
use crash_handler::CrashHandler;
use libobs_wrapper::{context::ObsContext, data::ObsData, encoders::ObsContextEncoders, utils::{AudioEncoderInfo, ObsPath, OutputInfo, StartupInfo, VideoEncoderInfo}};
use std::{
    env::{args, current_dir, current_exe}, fs::File, io::Write, path::PathBuf, thread, time::{Duration, SystemTime, UNIX_EPOCH}
};

/// Attaches a crash handler to the client process
///
/// Returns a CrashHandler that must be kept alive until the program exits.
/// Dropping the handler will detach it.
///
/// If you need this on non-Windows, re-visit
/// <https://github.com/EmbarkStudios/crash-handling/blob/main/minidumper/examples/diskwrite.rs>
/// Linux has a special `set_ptracer` call that is handy
/// MacOS needs a special `ping` call to flush messages inside the crash handler
pub(crate) fn attach_handler() -> Result<CrashHandler> {
    // Attempt to connect to the server
    let (client, _server) = start_server_and_connect()?;

    // SAFETY: Unsafe is required here because this will run after the program
    // has crashed. We should try to do as little as possible, basically just
    // tell the crash handler process to get our minidump and then return.
    // https://docs.rs/crash-handler/0.6.0/crash_handler/trait.CrashEvent.html#safety
    let handler = CrashHandler::attach(unsafe {
        crash_handler::make_crash_event(move |crash_context| {
            let handled = client.request_dump(crash_context).is_ok();
            eprintln!("Firezone crashed and wrote a crash dump.");
            crash_handler::CrashEventResult::Handled(handled)
        })
    })
    .context("failed to attach signal handler")?;

    Ok(handler)
}

/// Main function for the server process, for out-of-process crash handling.
///
/// The server process seems to be the preferred method,
/// since it's hard to run complex code in a process
/// that's already crashed and likely suffered memory corruption.
///
/// <https://jake-shadle.github.io/crash-reporting/#implementation>
/// <https://chromium.googlesource.com/breakpad/breakpad/+/master/docs/getting_started_with_breakpad.md#terminology>
pub(crate) fn server(socket_path: PathBuf) -> Result<()> {
    let mut server = minidumper::Server::with_name(&*socket_path)?;
    let ab = std::sync::atomic::AtomicBool::new(false);
    server.run(Box::new(Handler::default()), &ab, None)?;
    Ok(())
}

fn start_server_and_connect() -> Result<(minidumper::Client, std::process::Child)> {
    let exe = std::env::current_exe().context("unable to find our own exe path")?;
    // Path of a Unix domain socket for IPC with the crash handler server
    // <https://github.com/EmbarkStudios/crash-handling/issues/10>
    let socket_path = current_dir()
        .context("`known_dirs::runtime` failed")?
        .join("crash_handler_pipe");
    std::fs::create_dir_all(
        socket_path
            .parent()
            .context("`known_dirs::runtime` should have a parent")?,
    )
    .context("Failed to create dir for crash_handler_pipe")?;

    let mut server = None;

    // I don't understand why there's a loop here. The original was an infinite loop,
    // so I reduced it to 10 and it still worked.
    // <https://github.com/EmbarkStudios/crash-handling/blob/16c2545f2a46b6b21d1e401cfeaf0d5b9a130b08/minidumper/examples/diskwrite.rs#L72>
    for _ in 0..10 {
        // Create the crash client first so we can error out if another instance of
        // the Firezone client is already using this socket for crash handling.
        if let Ok(client) = minidumper::Client::with_name(&*socket_path) {
            return Ok((
                client,
                server.ok_or_else(|| {
                    anyhow!(
                        "should be impossible to make a client if we didn't make the server yet"
                    )
                })?,
            ));
        }

        server = Some(
            std::process::Command::new(&exe)
                .arg("crash-handler-server")
                .arg(&socket_path)
                .spawn()
                .context("unable to spawn server process")?,
        );

        // Give it time to start
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    bail!("Couldn't set up crash handler server")
}

/// Crash handler that runs inside the crash handler process.
///
/// The minidumper docs call this the "server" process because it's an IPC server,
/// not to be confused with network servers for Firezone itself.
struct Handler;

impl Default for Handler {
    fn default() -> Self {
        // Capture the time at startup so that the crash dump file will have
        // a similar timestamp to the log file
        Self {}
    }
}

impl minidumper::ServerHandler for Handler {
    /// Called when a crash has been received and a backing file needs to be
    /// created to store it.
    #[allow(clippy::print_stderr)]
    fn create_minidump_file(&self) -> Result<(File, PathBuf), std::io::Error> {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        let date = since_the_epoch.to_string();
        let dump_path = current_dir()
            .expect("Should be able to find logs dir to put crash dump in")
            .join(format!("crash.{date}.dmp"));

        // `tracing` is unlikely to work inside the crash handler subprocess, so
        // just print to stderr and it may show up on the terminal. This helps in CI / local dev.
        eprintln!("Creating minidump at {}", dump_path.display());
        let Some(dir) = dump_path.parent() else {
            return Err(std::io::ErrorKind::NotFound.into());
        };
        std::fs::create_dir_all(dir)?;
        let file = File::create(&dump_path)?;
        Ok((file, dump_path))
    }

    /// Called when a crash has been fully written as a minidump to the provided
    /// file. Also returns the full heap buffer as well.
    fn on_minidump_created(
        &self,
        result: Result<minidumper::MinidumpBinary, minidumper::Error>,
    ) -> minidumper::LoopAction {
        match result {
            Ok(mut md_bin) => {
                let _ = md_bin.file.flush();
                // Copy the timestamped crash file to a well-known filename,
                // this makes it easier for the smoke test to find it
                std::fs::copy(
                    &md_bin.path,
                    current_dir()
                        .expect("Should be able to find logs dir")
                        .join("last_crash.dmp"),
                )
                .ok();
                println!("wrote minidump to disk");
            }
            Err(e) => {
                eprintln!("failed to write minidump: {:#}", e);
            }
        }

        // Tells the server to exit, which will in turn exit the process
        minidumper::LoopAction::Exit
    }

    fn on_message(&self, kind: u32, buffer: Vec<u8>) {
        println!(
            "kind: {kind}, message: {}",
            String::from_utf8(buffer).expect("message should be valid UTF-8")
        );
    }

    fn on_client_disconnected(&self, num_clients: usize) -> minidumper::LoopAction {
        if num_clients == 0 {
            minidumper::LoopAction::Exit
        } else {
            minidumper::LoopAction::Continue
        }
    }
}

fn main() {
    let a = args().nth(1);
    if a.is_some_and(|e| e == "crash-handler-server") {
        let socket_path = args().nth(2).expect("expected socket path");
        let socket_path = PathBuf::from(socket_path);

        server(socket_path).expect("server failed");
        return;
    }

    // Attempt to connect to the server
    let (client, _server) = start_server_and_connect().unwrap();

    let _handler = CrashHandler::attach(unsafe {
        crash_handler::make_crash_event(move |crash_context| {
            let handled = client.request_dump(crash_context).is_ok();
            eprintln!("Firezone crashed and wrote a crash dump.");
            crash_handler::CrashEventResult::Handled(handled)
        })
    })
    .context("failed to attach signal handler")
    .unwrap();

    println!("{:?} {:?}", current_dir(), current_exe());
    run_basic_obs().unwrap();
}

pub fn run_basic_obs() -> Result<()> {
    env_logger::init();

    // Start the OBS context
    let startup_info = StartupInfo::default();
    let mut context = ObsContext::new(startup_info).unwrap();

    // Set up output to ./recording.mp4
    let mut output_settings = ObsData::new();
    output_settings.set_string("path", ObsPath::from_relative("recording.mp4").build());

    let output_info = OutputInfo::new("ffmpeg_muxer", "output", Some(output_settings), None);

    let output = context.output(output_info).unwrap();

    // Register the video encoder
    let mut video_settings = ObsData::new();
    video_settings
        .set_int("bf", 2)
        .set_bool("psycho_aq", true)
        .set_bool("lookahead", true)
        .set_string("profile", "high")
        .set_string("preset", "hq")
        .set_string("rate_control", "cbr")
        .set_int("bitrate", 10000);

    let video_info = VideoEncoderInfo::new(
        ObsContext::get_best_video_encoder(),
        "video_encoder",
        Some(video_settings),
        None,
    );

    let video_handler = ObsContext::get_video_ptr().unwrap();
    output.video_encoder(video_info, video_handler)?;

    // Register the audio encoder
    let mut audio_settings = ObsData::new();
    audio_settings.set_int("bitrate", 160);

    let audio_info =
        AudioEncoderInfo::new("ffmpeg_aac", "audio_encoder", Some(audio_settings), None);

    let audio_handler = ObsContext::get_audio_ptr().unwrap();
    output.audio_encoder(audio_info, 0, audio_handler)?;

    // Create the video source using game capture
    let mut video_source_data = ObsData::new();
    video_source_data
        .set_string("capture_mode", "window")
        .set_string("window", "")
        .set_bool("capture_cursor", true);

    /*let video_source_info = SourceInfo::new(
        "game_capture",
        "video_source",
        Some(video_source_data),
        None,
    );
    */

    // Register the source and record
    //output.source(video_source_info, 0)?;
    output.start()?;

    println!("recording for 10 seconds...");
    thread::sleep(Duration::new(10, 0));

    // Open any fullscreen application and
    // Success!
    output.stop()?;

    Ok(())
}
