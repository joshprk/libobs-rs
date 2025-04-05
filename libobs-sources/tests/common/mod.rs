mod initialize;

use std::path::Path;

use anyhow::bail;
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

fn parse_ffmpeg_duration(duration: &str) -> anyhow::Result<f64> {
    let parts: Vec<&str> = duration.split(':').collect();
    if parts.len() != 3 {
        bail!("Invalid duration format");
    }

    let hours: f64 = parts[0].parse()?;
    let minutes: f64 = parts[1].parse()?;
    let seconds: f64 = parts[2].parse()?;

    let total_seconds = hours * 3600.0 + minutes * 60.0 + seconds;
    Ok(total_seconds)
}

pub async fn test_video(vid_path: &Path, divider: f64) -> anyhow::Result<()> {
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

    let stdout = format!("{}\n{}", String::from_utf8_lossy(&cmd.stdout), String::from_utf8_lossy(&cmd.stderr));
    let stdout = stdout.replace("\r", "");

    let duration = stdout
        .split("\n")
        .find(|l| l.contains("Duration"))
        .ok_or_else(|| anyhow::anyhow!("Couldn't find duration"))?
        .trim();

    let duration = duration.split(" ").collect::<Vec<_>>()[1];
    let duration = duration.replace(",", "");
    let duration = parse_ffmpeg_duration(&duration)?;

    let split = stdout
        .split("\n")
        .find(|l| l.contains("black_start"));
    if split.is_none() {
        // No black frames found,
        return Ok(())
    }


    let split = split.unwrap().trim()
        .split("]")
        .nth(1)
        .expect("Couldn't find black_start");

        println!("Split {:?}", split);
    let comps = split.split(" ").into_iter().collect::<Vec<_>>();

    let black_duration = comps
        .get(2)
        .ok_or_else(|| anyhow::anyhow!("Couldn't find duration"))?
        .split(":")
        .nth(1)
        .expect("Couldn't find black duration");

    let black_duration = black_duration.parse::<f64>()?;

    let max_no_black = 0.7 / divider;

    if black_duration / duration > max_no_black {
        return Err(anyhow::anyhow!("Black duration too long, Invalid video"));
    }

    Ok(())
}
