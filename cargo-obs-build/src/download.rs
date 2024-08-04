use std::{
    fs::File,
    io::{stdout, BufReader, Write},
    path::{Path, PathBuf},
    sync::mpsc::{self},
    thread,
    time::{Duration, Instant},
};

use anyhow::{anyhow, bail};
use colored::Colorize;
use http_req::{
    chunked::ChunkReader,
    request::{Request, RequestBuilder},
    response::Response,
    stream::{Stream, ThreadReceive, ThreadSend},
    uri::Uri,
};
use indicatif::{ProgressBar, ProgressStyle};
use sha2::{Digest, Sha256};

use crate::git::ReleaseInfo;

const DEFAULT_REQ_TIMEOUT: u64 = 60 * 60;


pub fn download_binaries(build_dir: &Path, info: &ReleaseInfo) -> anyhow::Result<PathBuf> {
    let to_download = &info.assets.iter().find(|e| {
        let name = e["name"].as_str().unwrap_or("").to_lowercase();

        // OBS-Studio-30.2.1-Windows.zip
        name.contains("obs-studio")
            && (name.contains("windows") || name.contains("full"))
            && name.contains(".zip")
            && !name.contains("pdb")
    });

    if to_download.is_none() {
        bail!("No OBS Studio binaries found");
    }

    let to_download = to_download.unwrap();
    let url = to_download["browser_download_url"]
        .as_str()
        .ok_or(anyhow!("No download url found"))?;

        let download_path = build_dir.join("obs-prebuilt-windows.zip");
    let hash = download_file(
        url,
        &download_path,
    )?;

    let name = to_download["name"].as_str().unwrap_or("");
    let checksum = &info.checksums.get(&name.to_lowercase());

    if let Some(checksum) = checksum {
        if checksum.to_lowercase() != hash.to_lowercase() {
            bail!("Checksums do not match");
        } else {
            println!("{}", "Checksums match".on_green());
        }
    } else {
        eprintln!("No checksum found for {}", name);
    }

    Ok(download_path)
}

pub fn get_download_url(url: &str) -> anyhow::Result<String> {
    let url = Uri::try_from(url)?;

    let mut body = Vec::new();
    let res = Request::new(&url)
        .header("User-Agent", "cargo-obs-build")
        .method(http_req::request::Method::HEAD)
        .send(&mut body)?;

    let s = res.status_code();
    if !s.is_redirect() {
        return Ok(url.to_string());
    }

    let location = res.headers().get("Location");
    if s.is_redirect() && location.is_some() {
        return Ok(location.unwrap().to_string());
    }

    bail!("Couldn't get redirect location (status {})", s)
}

/// Returns hash
pub fn download_file(url: &str, path: &Path) -> anyhow::Result<String> {
    let timeout = Duration::from_secs(60);
    let url = get_download_url(url)?;
    let uri = Uri::try_from(url.as_str())?;
    let mut stream = Stream::new(&uri, Some(timeout.clone()))?;

    stream.set_read_timeout(Some(timeout.clone()))?;
    stream.set_write_timeout(Some(timeout.clone()))?;

    stream = Stream::try_to_https(stream, &uri, None)?;

    let res = RequestBuilder::new(&uri)
        .header("Connection", "Close")
        .header("User-Agent", "cargo-obs-build")
        .parse();
    stream.write_all(&res)?;

    // Set up variables
    let (sender, receiver) = mpsc::channel();
    let (sender_supp, receiver_supp) = mpsc::channel();
    let mut raw_response_head: Vec<u8> = Vec::new();
    let mut buf_reader = BufReader::new(stream);

    // Read from the stream and send over data via `sender`.
    thread::spawn(move || {
        buf_reader.send_head(&sender);

        let params: Vec<&str> = receiver_supp.recv().unwrap();
        //TODO this never exists
        if params.contains(&"chunked") {
            let mut buf_reader = ChunkReader::from(buf_reader);
            buf_reader.send_all(&sender);
        } else {
            buf_reader.send_all(&sender);
        }

    });

    let deadline = Instant::now() + Duration::from_secs(DEFAULT_REQ_TIMEOUT);

    // Receive and process `head` of the response.
    raw_response_head.receive(&receiver, deadline)?;

    let response = Response::from_head(&raw_response_head)?;
    let content_len = response.content_len().unwrap_or(1) as u64;
    let encoding = response.headers().get("Transfer-Encoding");
    let mut params = Vec::with_capacity(4);

    if let Some(encode) = encoding {
        if encode == "chunked" {
            params.push("chunked");
        }
    }

    sender_supp.send(params).unwrap();

    if content_len <= 0 {
        bail!("Content length is 0");
    }

    let style = ProgressStyle::default_bar()
    .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
    .map_err(|e| anyhow!("Couldn't create style {:#?}", e))?
    .progress_chars("#>-");

    let pb = ProgressBar::new(content_len);
    pb.set_style(style);
    pb.set_message(format!("Downloading OBS binaries"));

    let mut file =
        File::create(path).or(Err(anyhow!("Failed to create file '{}'", path.display())))?;
    let mut downloaded: u64 = 0;

    let mut hasher = Sha256::new();
    loop {
        let now = Instant::now();
        let remaining_time = deadline - now;

        let item = receiver.recv_timeout(remaining_time);
        if let Err(_e) = item {
            break;
        }

        let chunk = item.unwrap();

        hasher.write_all(&chunk)?;
        file.write_all(&chunk)
            .or(Err(anyhow!("Error while writing to file")))?;

        let new = std::cmp::min(downloaded + (chunk.len() as u64), content_len);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(format!("Downloaded OBS to {}", path.display()));
    println!("Hashing...");
    stdout().flush().unwrap();
    return Ok(hex::encode(hasher.finalize()));
}