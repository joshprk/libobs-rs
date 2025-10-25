use std::{
    env::current_exe,
    path::{Path, PathBuf},
};

use async_stream::stream;
use futures_core::Stream;
use futures_util::{StreamExt, pin_mut};
use sevenz_rust::{Password, SevenZReader, default_entry_extract_fn};
use tokio::task;
pub enum ExtractStatus {
    Error(anyhow::Error),
    Progress(f32, String),
}

pub(crate) async fn extract_obs(
    archive_file: &Path,
) -> anyhow::Result<impl Stream<Item = ExtractStatus>> {
    log::info!("Extracting OBS at {}", archive_file.display());

    let path = PathBuf::from(archive_file);

    let destination = current_exe()?;
    let destination = destination
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Should be able to get parent of exe"))?
        .join("obs_new");

    //TODO delete old obs dlls and plugins
    let dest = destination.clone();
    let stream = stream! {
        yield Ok((0.0, "Reading file...".to_string()));
        let mut sz = SevenZReader::open(&path, Password::empty())?;
        let (tx, mut rx) = tokio::sync::mpsc::channel(5);

        let total = sz.archive().files.len() as f32;
        if !dest.exists() {
            std::fs::create_dir_all(&dest)?;
        }

        let mut curr = 0;
        let mut r = task::spawn_blocking(move || {
            sz.for_each_entries(|entry, reader| {
                curr += 1;
                tx.blocking_send((curr as f32 / total, format!("Extracting {}", entry.name()))).unwrap();

                let dest_path = dest.join(entry.name());

                default_entry_extract_fn(entry, reader, &dest_path)
            })?;

            Result::<_, anyhow::Error>::Ok((1.0, "Extraction done".to_string()))
        });

        loop {
            tokio::select! {
                m = rx.recv() => {
                    match m {
                        Some(e) => yield Ok(e),
                        None => break
                    }
                },
                res = &mut r => {
                    match res {
                        Ok(e) => yield e,
                        Err(e) => {
                            yield Err(e.into());
                        }
                    }

                    break;
                }
            };
        }

        yield Ok((1.0, "Extraction done".to_string()));
    };

    Ok(stream! {
            pin_mut!(stream);
            while let Some(status) = stream.next().await {
                match status {
                    Ok(e) => yield ExtractStatus::Progress(e.0, e.1),
                    Err(err) => {
                        log::error!("Error extracting OBS: {:?}", err);
                        yield ExtractStatus::Error(err);
                        return;
                    }
                }
            }

    })
}
