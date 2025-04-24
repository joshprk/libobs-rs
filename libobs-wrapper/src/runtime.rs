//! This runtime creates a thread to manage OBS, so it can be used across threads

use std::ffi::CStr;
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::{fmt::Debug, thread::JoinHandle};
use std::{ptr, thread};
use tokio::sync::{oneshot, Mutex};

use crate::bootstrap::bootstrap;
use crate::crash_handler::main_crash_handler;
use crate::enums::{ObsLogLevel, ObsResetVideoStatus};
use crate::logger::{extern_log_callback, internal_log_global, LOGGER};
use crate::utils::initialization::load_debug_privilege;
use crate::utils::{ObsBootstrapError, ObsError, ObsModules, ObsString};
use crate::{
    context::OBS_THREAD_ID,
    utils::StartupInfo,
};

// Command type for operations to perform on the OBS thread
enum ObsCommand {
    Execute(
        Box<dyn FnOnce() -> Box<dyn std::any::Any + Send> + Send>,
        oneshot::Sender<Box<dyn std::any::Any + Send>>,
    ),
    Terminate,
}

pub enum ObsRuntimeReturn {
    /// The OBS context is ready to use
    Done((ObsRuntime, ObsModules, StartupInfo)),

    /// The application must be restarted to apply OBS updates
    Restart,
}

#[derive(Debug, Clone)]
pub struct ObsRuntime {
    handle: Arc<Mutex<Option<JoinHandle<()>>>>,
    command_sender: Arc<Sender<ObsCommand>>,
}

#[derive(Debug)]
struct SendableModules(ObsModules);
unsafe impl Sync for SendableModules {}
unsafe impl Send for SendableModules {}

impl ObsRuntime {
    //! This runtime creates a thread to manage OBS, so it can be used across threads.
    //! Will startup OBS when this function is called.

    pub(crate) async fn startup(mut options: StartupInfo) -> Result<ObsRuntimeReturn, ObsError> {
        let obs_id = OBS_THREAD_ID.lock().await;
        if obs_id.is_some() {
            return Err(ObsError::ThreadFailure);
        }

        drop(obs_id);

        #[cfg(feature = "bootstrapper")]
        if options.bootstrap_handler.is_some() {
            use crate::bootstrap::BootstrapStatus;
            use futures_util::{pin_mut, StreamExt};

            log::trace!("Starting bootstrapper");
            let stream = bootstrap(&options.bootstrapper_options)
                .await
                .map_err(|e| {
                    ObsError::BootstrapperFailure(ObsBootstrapError::GeneralError(e.to_string()))
                })?;
            if let Some(stream) = stream {
                pin_mut!(stream);

                log::trace!("Waiting for bootstrapper to finish");
                while let Some(item) = stream.next().await {
                    match item {
                        BootstrapStatus::Downloading(progress, message) => {
                            if let Some(handler) = &mut options.bootstrap_handler {
                                handler
                                    .handle_downloading(progress, message)
                                    .await
                                    .map_err(|e| {
                                        ObsError::BootstrapperFailure(
                                            ObsBootstrapError::DownloadError(e.to_string()),
                                        )
                                    })?;
                            }
                        }
                        BootstrapStatus::Extracting(progress, message) => {
                            if let Some(handler) = &mut options.bootstrap_handler {
                                handler.handle_extraction(progress, message).await.map_err(
                                    |e| {
                                        ObsError::BootstrapperFailure(
                                            ObsBootstrapError::ExtractError(e.to_string()),
                                        )
                                    },
                                )?;
                            }
                        }
                        BootstrapStatus::Error(err) => {
                            return Err(ObsError::BootstrapperFailure(
                                ObsBootstrapError::GeneralError(err.to_string()),
                            ));
                        }
                        BootstrapStatus::RestartRequired => {
                            return Ok(ObsRuntimeReturn::Restart);
                        }
                    }
                }
            }
        }

        log::trace!("Initializing OBS context");
        return Ok(ObsRuntimeReturn::Done(
            ObsRuntime::init(options).await.map_err(|e| {
                ObsError::BootstrapperFailure(ObsBootstrapError::GeneralError(e.to_string()))
            })?,
        ));
    }

    async fn init(info: StartupInfo) -> anyhow::Result<(ObsRuntime, ObsModules, StartupInfo)> {
        let (command_sender, command_receiver) = channel();
        let (init_tx, init_rx) = oneshot::channel();

        let handle = std::thread::spawn(move || {
            log::trace!("Starting OBS thread");

            let res = Self::initialize_inner(info);

            match res {
                Ok((info, modules)) => {
                    log::trace!("OBS context initialized successfully");
                    let e = init_tx.send(Ok((SendableModules(modules), info)));
                    if let Err(err) = e {
                        log::error!("Failed to send initialization signal: {:?}", err);
                    }

                    // Process commands until termination
                    while let Ok(command) = command_receiver.recv() {
                        match command {
                            ObsCommand::Execute(func, result_sender) => {
                                let result = func();
                                let _ = result_sender.send(result);
                            }
                            ObsCommand::Terminate => break,
                        }
                    }

                    Self::shutdown_inner();
                }
                Err(err) => {
                    log::error!("Failed to initialize OBS context: {:?}", err);
                    let _ = init_tx.send(Err(err));
                }
            }
        });

        log::trace!("Waiting for OBS thread to initialize");
        // Wait for initialization to complete
        let (mut m, info) = init_rx.await??;

        let runtime = Self {
            handle: Arc::new(Mutex::new(Some(handle))),
            command_sender: Arc::new(command_sender),
        };

        m.0.runtime = Some(runtime.clone());
        Ok((runtime, m.0, info))
    }

    /// Run a function with the ObsContext
    ///
    /// This allows you to execute operations on the OBS thread that don't return a value
    pub async fn run_with_obs<F>(&self, operation: F) -> anyhow::Result<()>
    where
        F: FnOnce() -> () + Send + 'static,
    {
        self.run_with_obs_result(move || {
            operation();
            Result::<(), anyhow::Error>::Ok(())
        })
        .await??;

        Ok(())
    }

    /// Run a function with the ObsContext and get a result
    ///
    /// This allows you to execute operations on the OBS thread and get a value back
    /// # Panics
    ///
    /// This function panics if called within an asynchronous execution
    /// context.
    pub fn run_with_obs_result_blocking<F, T>(&self, operation: F) -> anyhow::Result<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = oneshot::channel();

        // Create a wrapper closure that boxes the result as Any
        let wrapper = move || -> Box<dyn std::any::Any + Send> {
            let result = operation();
            Box::new(result)
        };

        self.command_sender
            .send(ObsCommand::Execute(Box::new(wrapper), tx))
            .map_err(|_| anyhow::anyhow!("Failed to send command to OBS thread"))?;

        let result = rx
            .blocking_recv()
            .map_err(|_| anyhow::anyhow!("OBS thread dropped the response channel"))?;

        // Downcast the Any type back to T
        result
            .downcast::<T>()
            .map(|boxed| *boxed)
            .map_err(|_| anyhow::anyhow!("Failed to downcast result to the expected type"))
    }

    /// Run a function with the ObsContext and get a result
    ///
    /// This allows you to execute operations on the OBS thread and get a value back
    pub async fn run_with_obs_result<F, T>(&self, operation: F) -> anyhow::Result<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = oneshot::channel();

        // Create a wrapper closure that boxes the result as Any
        let wrapper = move || -> Box<dyn std::any::Any + Send> {
            let result = operation();
            Box::new(result)
        };

        self.command_sender
            .send(ObsCommand::Execute(Box::new(wrapper), tx))
            .map_err(|_| anyhow::anyhow!("Failed to send command to OBS thread"))?;

        let result = rx
            .await
            .map_err(|_| anyhow::anyhow!("OBS thread dropped the response channel"))?;

        // Downcast the Any type back to T
        result
            .downcast::<T>()
            .map(|boxed| *boxed)
            .map_err(|_| anyhow::anyhow!("Failed to downcast result to the expected type"))
    }

    /// Shutdown the OBS runtime and terminate the thread
    pub async fn shutdown(self) -> anyhow::Result<()> {
        self.command_sender
            .send(ObsCommand::Terminate)
            .map_err(|_| anyhow::anyhow!("Failed to send termination command to OBS thread"))?;

        // Wait for the thread to finish
        let mut handle = self.handle.lock().await;
        let handle = handle.take().expect("Handle can not be empty");

        if let Err(err) = handle.join() {
            return Err(anyhow::anyhow!("OBS thread panicked: {:?}", err));
        }
        Ok(())
    }

    /// Initializes the libobs context and prepares
    /// it for recording.
    ///
    /// More specifically, it calls `obs_startup`,
    /// `obs_reset_video`, `obs_reset_audio`, and
    /// registers the video and audio encoders.
    ///
    /// At least on Windows x64, it seems that
    /// resetting video and audio is necessary to
    /// prevent a memory leak when restarting the
    /// OBS context. This memory leak is not severe
    /// (~10 KB per restart), but the point is
    /// safety. Thank you @tt2468 for the help!
    fn initialize_inner(mut info: StartupInfo) -> Result<(StartupInfo, ObsModules), ObsError> {
        // Checks that there are no other threads
        // using libobs using a static Mutex.
        //
        // Fun fact: this code caused a huge debate
        // about whether AtomicBool is UB or whatever
        // in the Rust Programming Discord server.
        // I didn't read too closely into it because
        // they were talking about what architecture
        // fridges have or something.
        //
        // Since this function is not meant to be
        // high-performance or called a thousand times,
        // a Mutex is fine here.
        let mut mutex_value = OBS_THREAD_ID.blocking_lock();

        // Directly checks if the value of the
        // Mutex is false. If true, then error.
        // We've checked already but keeping this
        if *mutex_value != None {
            return Err(ObsError::ThreadFailure);
        }

        // If the Mutex is None, then change
        // it to current thread ID so that no
        // other thread can use libobs while
        // the current thread is using it.
        *mutex_value = Some(thread::current().id());

        // Install DLL blocklist hook here

        unsafe {
            libobs::obs_init_win32_crash_handler();
        }

        // Set logger, load debug privileges and crash handler
        unsafe {
            libobs::base_set_crash_handler(Some(main_crash_handler), std::ptr::null_mut());
            load_debug_privilege();
            libobs::base_set_log_handler(Some(extern_log_callback), std::ptr::null_mut());
        }

        let mut log_callback = LOGGER.lock().map_err(|_e| ObsError::MutexFailure)?;

        *log_callback = info.logger.take().expect("Logger can never be null");
        drop(log_callback);

        // Locale will only be used internally by
        // libobs for logging purposes, making it
        // unnecessary to support other languages.
        let locale_str = ObsString::new("en-US");
        let startup_status =
            unsafe { libobs::obs_startup(locale_str.as_ptr().0, ptr::null(), ptr::null_mut()) };

        let version = unsafe { libobs::obs_get_version_string() };
        let version_cstr = unsafe { CStr::from_ptr(version) };
        let version_str = version_cstr.to_string_lossy().into_owned();

        internal_log_global(ObsLogLevel::Info, format!("OBS {}", version_str));
        internal_log_global(
            ObsLogLevel::Info,
            "---------------------------------".to_string(),
        );

        if !startup_status {
            return Err(ObsError::Failure);
        }

        let mut obs_modules = ObsModules::add_paths(&info.startup_paths);

        // Note that audio is meant to only be reset
        // once. See the link below for information.
        //
        // https://docs.obsproject.com/frontends
        unsafe {
            libobs::obs_reset_audio2(info.obs_audio_info.as_ptr().0);
        }

        // Resets the video context. Note that this
        // is similar to Self::reset_video, but it
        // does not call that function because the
        // ObsContext struct is not created yet,
        // and also because there is no need to free
        // anything tied to the OBS context.
        let reset_video_status = num_traits::FromPrimitive::from_i32(unsafe {
            libobs::obs_reset_video(info.obs_video_info.as_ptr())
        });

        let reset_video_status = match reset_video_status {
            Some(x) => x,
            None => ObsResetVideoStatus::Failure,
        };

        if reset_video_status != ObsResetVideoStatus::Success {
            return Err(ObsError::ResetVideoFailure(reset_video_status));
        }

        obs_modules.load_modules();

        internal_log_global(
            ObsLogLevel::Info,
            "==== Startup complete ===============================================".to_string(),
        );

        Ok((info, obs_modules))
    }

    fn shutdown_inner() {
        // Clean up sources
        for i in 0..libobs::MAX_CHANNELS {
            unsafe { libobs::obs_set_output_source(i, ptr::null_mut()) };
        }

        unsafe { libobs::obs_shutdown() }

        let r = LOGGER.lock();
        match r {
            Ok(mut logger) => {
                logger.log(ObsLogLevel::Info, "OBS context shutdown.".to_string());
                let allocs = unsafe { libobs::bnum_allocs() };

                // Increasing this to 1 because of whats described below
                let level = if allocs > 1 {
                    ObsLogLevel::Error
                } else {
                    ObsLogLevel::Info
                };
                // One memory leak is expected here because OBS does not free array elements of the obs_data_path when calling obs_add_data_path
                // even when obs_remove_data_path is called. This is a bug in OBS.
                logger.log(level, format!("Number of memory leaks: {}", allocs))
            }
            Err(_) => {
                println!("OBS context shutdown. (but couldn't lock logger)");
            }
        }

        unsafe {
            // Clean up log and crash handler
            libobs::base_set_crash_handler(None, std::ptr::null_mut());
            libobs::base_set_log_handler(None, std::ptr::null_mut());
        }

        let mut mutex_value = OBS_THREAD_ID.blocking_lock();
        *mutex_value = None;
    }
}
