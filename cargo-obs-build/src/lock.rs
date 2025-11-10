use std::{
    fs::{self, File},
    io::{stdout, Read, Write},
    path::{Path, PathBuf},
};

use process_alive::Pid;

pub fn wait_for_lock(lock: &Path) -> anyhow::Result<()> {
    if !lock.is_file() {
        return Ok(());
    }

    let mut f = File::open(lock)?;
    let mut pid = String::new();

    f.read_to_string(&mut pid)?;
    let pid = pid
        .trim()
        .parse::<u32>()
        .map_err(|e| anyhow::anyhow!("Failed to parse PID from lock file: {}", e));

    if pid.is_err() {
        fs::remove_file(lock)?;
        return Ok(());
    }

    let pid = Pid::from(pid?);
    let state = process_alive::state(pid);

    if state.is_alive() {
        println!("Another instance is already running, waiting");
        while process_alive::state(pid).is_alive() {
            std::thread::sleep(std::time::Duration::from_secs(1));
            print!(".");
            stdout().flush()?;
        }

        println!();
    }
    Ok(())
}

pub struct LockGuard {
    lock: PathBuf,
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        let e = fs::remove_file(&self.lock);
        if cfg!(feature = "cli") {
            e.unwrap();
        } else {
            eprintln!(
                "cargo:warning=Failed to remove lock file: {}",
                e.unwrap_err()
            );
        }
    }
}

pub fn acquire_lock(lock: &Path) -> anyhow::Result<LockGuard> {
    let pid = std::process::id().to_string();
    fs::write(lock, pid)?;

    Ok(LockGuard {
        lock: lock.to_path_buf(),
    })
}
