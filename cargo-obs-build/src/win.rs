use std::{
    fs::{self, File},
    path::Path,
};

use colored::Colorize;
use walkdir::WalkDir;
use zip::ZipArchive;

use crate::{
    download::download_binaries,
    git::ReleaseInfo,
    util::{copy_to_dir, get_cmake_build, get_rundir_out},
};

pub fn process_source(
    repo_dir: &Path,
    build_out: &Path,
    obs_preset: &str,
    // Whether to download the OBS Studio binaries
    download_bin: bool,
    info: &ReleaseInfo,
) -> anyhow::Result<()> {
    let cmake_run_dir_out = get_rundir_out(repo_dir, obs_preset);

    if download_bin {
        replace_with_signed_bins(&repo_dir, &cmake_run_dir_out, info)?;
    }

    println!(
        "Copying files from {} to {}",
        cmake_run_dir_out.display(),
        build_out.display()
    );

    let bin_dir = cmake_run_dir_out.join("bin");
    let bin_arch_dir = bin_dir.join("64bit");

    copy_to_dir(&bin_arch_dir, &build_out, None)?;
    copy_to_dir(&cmake_run_dir_out, &build_out, Some(&bin_dir))?;

    Ok(())
}

fn replace_with_signed_bins(
    repo_dir: &Path,
    run_dir: &Path,
    info: &ReleaseInfo,
) -> anyhow::Result<()> {
    let cmake_build_out = get_cmake_build(repo_dir);

    fs::create_dir_all(&cmake_build_out)?;
    let obs_archive = download_binaries(&cmake_build_out, info)?;

    let iter = WalkDir::new(run_dir)
        .into_iter() //
        .filter_map(|e| e.ok());

    println!("Finding executables in {:?}", &run_dir);
    let mut to_replace = Vec::new();
    for entry in iter {
        let f = entry.file_name().to_string_lossy();
        if f.ends_with(".exe") {
            let p = entry.into_path();
            to_replace.push(p.strip_prefix(run_dir)?.to_path_buf());
        }
    }

    // Replace executables
    let obs_archive = File::open(&obs_archive)?;
    let mut archive = ZipArchive::new(&obs_archive)?;

    for entry in to_replace {
        let path = entry.display().to_string();
        let f = archive.by_name(&path.replace("\\", "/"));

        if let Err(e) = f {
            eprintln!("Failed to find {} in archive ({:?})", path.red(), e);
            continue;
        }

        let mut f = f?;
        let dest = run_dir.join(&entry);

        let mut dest = File::create(&dest)?;
        std::io::copy(&mut f, &mut dest)?;
    }

    println!("Replaced executables with {} versions", "signed".green());
    Ok(())
}
