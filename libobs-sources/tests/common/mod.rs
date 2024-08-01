mod initialize;

use std::{
    io::Read,
    path::{Path, PathBuf},
};

use essi_ffmpeg::FFmpeg;
pub use initialize::*;
use tokio::process::Command;

pub async fn check_ffmpeg() -> anyhow::Result<()> {
    // Automatically download FFmpeg if not found
    if let Some((handle, mut progress)) = FFmpeg::auto_download().await? {
        tokio::spawn(async move {
            while let Some(state) = progress.recv().await {
                println!("Downloading: {:?}", state);
            }
        });

        handle.await??;
    } else {
        println!("FFmpeg is downloaded, using existing installation");
    }

    Ok(())
}

pub async fn test_video(vid_path: &Path) -> anyhow::Result<()> {
    check_ffmpeg().await?;

    let prog = FFmpeg::get_program()?.ok_or_else(|| anyhow::anyhow!("Couldn't find FFmpeg"))?;
    let cmd = Command::new(prog)
        .arg("-i")
        .arg(vid_path)
        .arg("-vf")
        .arg("blackdetect=d=0:pic_th=0.98")
        .arg("-an")
        .arg("-f")
        .arg("null")
        .arg("-")
        .output()
        .await?;

    let stdout = String::from_utf8_lossy(&cmd.stdout);
    let stdout = stdout.replace("\r", "");

    let duration = stdout
        .split("\n")
        .find(|l| l.contains("Duration"))
        .ok_or_else(|| anyhow::anyhow!("Couldn't find duration"))?;
    let duration = duration.split(" ").collect::<Vec<_>>()[1];
    let duration = duration.replace(",", "");
    let duration = duration.parse::<f64>()?;

    let split = stdout
        .split("\n")
        .find(|l| l.contains("black_start"))
        .ok_or_else(|| anyhow::anyhow!("Couldn't find black_start"))?;

    let comps = split.split(" ").into_iter().collect::<Vec<_>>();

    let black_duration = comps
        .get(2)
        .ok_or_else(|| anyhow::anyhow!("Couldn't find duration"))?;
    let black_duration = black_duration.parse::<f64>()?;

    let max_no_black = 0.7;

    if black_duration / duration > max_no_black {
        return Err(anyhow::anyhow!("Black duration too long, Invalid video"));
    }

    Ok(())
}
