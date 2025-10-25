use anyhow::bail;
use essi_ffmpeg::FFmpeg;
use std::path::Path;
use tokio::process::Command;

pub async fn check_ffmpeg() -> anyhow::Result<()> {
    // Automatically download FFmpeg if not found
    if let Some((handle, mut progress)) = FFmpeg::auto_download().await? {
        tokio::spawn(async move {
            while let Some(state) = progress.recv().await {
                println!("Downloading FFmpeg: {:?}", state);
            }
        });

        handle.await??;
    } else {
        println!("FFmpeg is available");
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

/// Test that a video is not black and has valid content
pub async fn test_video_not_black(vid_path: &Path, divider: f64) -> anyhow::Result<()> {
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

    let stdout = format!(
        "{}\n{}",
        String::from_utf8_lossy(&cmd.stdout),
        String::from_utf8_lossy(&cmd.stderr)
    );
    let stdout = stdout.replace("\r", "");

    let duration = stdout
        .split("\n")
        .find(|l| l.contains("Duration"))
        .ok_or_else(|| anyhow::anyhow!("Couldn't find duration"))?
        .trim();

    let duration = duration.split(" ").collect::<Vec<_>>()[1];
    let duration = duration.replace(",", "");
    let duration = parse_ffmpeg_duration(&duration)?;

    let split = stdout.split("\n").find(|l| l.contains("black_start"));
    if split.is_none() {
        // No black frames found
        return Ok(());
    }

    let split = split
        .unwrap()
        .trim()
        .split("]")
        .nth(1)
        .expect("Couldn't find black_start");

    println!("Black detection: {:?}", split);
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
        return Err(anyhow::anyhow!(
            "Black duration too long ({}/{} = {}), invalid video",
            black_duration,
            duration,
            black_duration / duration
        ));
    }

    Ok(())
}

/// Test that a video has motion by checking frame variance
pub async fn test_video_has_motion(vid_path: &Path) -> anyhow::Result<()> {
    check_ffmpeg().await?;

    let prog = FFmpeg::get_program()?.ok_or_else(|| anyhow::anyhow!("Couldn't find FFmpeg"))?;
    
    // Use freezedetect to detect static frames
    let cmd = Command::new(&prog)
        .arg("-i")
        .arg(vid_path)
        .arg("-vf")
        .arg("freezedetect=n=-60dB:d=0.5")
        .arg("-an")
        .arg("-f")
        .arg("null")
        .arg("-")
        .output()
        .await?;

    let output = format!(
        "{}\n{}",
        String::from_utf8_lossy(&cmd.stdout),
        String::from_utf8_lossy(&cmd.stderr)
    );
    let output = output.replace("\r", "");

    println!("Freeze detection output:\n{}", output);

    // Get total duration
    let duration_line = output
        .split("\n")
        .find(|l| l.contains("Duration"))
        .ok_or_else(|| anyhow::anyhow!("Couldn't find duration"))?;
    
    let duration_str = duration_line.split(" ").collect::<Vec<_>>()[1];
    let duration_str = duration_str.replace(",", "");
    let total_duration = parse_ffmpeg_duration(&duration_str)?;

    println!("Total video duration: {:.2}s", total_duration);

    // Check for freeze detection
    let freeze_lines: Vec<&str> = output
        .split("\n")
        .filter(|l| l.contains("lavfi.freezedetect"))
        .collect();

    if freeze_lines.is_empty() {
        println!("No frozen frames detected - video has motion!");
        return Ok(());
    }

    // Calculate total frozen time
    let mut total_frozen = 0.0;
    for line in freeze_lines {
        if line.contains("freeze_duration") {
            if let Some(duration_part) = line.split("freeze_duration:").nth(1) {
                if let Ok(duration) = duration_part.trim().parse::<f64>() {
                    total_frozen += duration;
                }
            }
        }
    }

    println!("Total frozen time: {:.2}s out of {:.2}s", total_frozen, total_duration);

    // If more than 80% of the video is frozen, it's not a valid motion test
    let frozen_ratio = total_frozen / total_duration;
    if frozen_ratio > 0.8 {
        return Err(anyhow::anyhow!(
            "Video appears to be mostly static ({:.1}% frozen)",
            frozen_ratio * 100.0
        ));
    }

    Ok(())
}

/// Get the frame count from a video
pub async fn get_frame_count(vid_path: &Path) -> anyhow::Result<u32> {
    check_ffmpeg().await?;

    let prog = FFmpeg::get_program()?.ok_or_else(|| anyhow::anyhow!("Couldn't find FFmpeg"))?;
    let cmd = Command::new(prog)
        .arg("-i")
        .arg(vid_path)
        .arg("-map")
        .arg("0:v:0")
        .arg("-c")
        .arg("copy")
        .arg("-f")
        .arg("null")
        .arg("-")
        .output()
        .await?;

    let output = format!(
        "{}\n{}",
        String::from_utf8_lossy(&cmd.stdout),
        String::from_utf8_lossy(&cmd.stderr)
    );

    // Look for frame count in output
    for line in output.lines() {
        if line.contains("frame=") {
            if let Some(frame_part) = line.split("frame=").nth(1) {
                if let Some(frame_str) = frame_part.split_whitespace().next() {
                    if let Ok(frames) = frame_str.trim().parse::<u32>() {
                        println!("Video has {} frames", frames);
                        return Ok(frames);
                    }
                }
            }
        }
    }

    Err(anyhow::anyhow!("Could not determine frame count"))
}
