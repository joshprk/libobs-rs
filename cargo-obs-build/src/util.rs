use std::{fs, path::Path};

use walkdir::WalkDir;

pub fn copy_to_dir(src: &Path, out: &Path, except_dir: Option<&Path>) -> anyhow::Result<()> {
    for entry in WalkDir::new(src) {
        if entry.is_err() {
            continue;
        }

        let entry = entry.unwrap();
        let path = entry.path();

        if except_dir.is_some_and(|e| path.starts_with(e)) {
            continue;
        }

        let copy_to = out.join(path.strip_prefix(src).unwrap());
        if path.is_dir() {
            fs::create_dir_all(&copy_to)?;
            continue;
        }

        fs::copy(entry.path(), copy_to)?;
    }

    Ok(())
}

pub fn delete_all_except(src: &Path, except_dir: Option<&Path>) -> anyhow::Result<()> {
    for entry in fs::read_dir(src)? {
        if entry.is_err() {
            continue;
        }

        let entry = entry.unwrap();
        let path = entry.path();

        if except_dir.is_some_and(|e| path.starts_with(e)) {
            continue;
        }

        if path.is_dir() {
            fs::remove_dir_all(path).unwrap();
        } else {
            fs::remove_file(path).unwrap();
        }
    }

    Ok(())
}
