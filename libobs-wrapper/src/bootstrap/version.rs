use std::path::Path;

use libobs::{LIBOBS_API_MAJOR_VER, LIBOBS_API_MINOR_VER, LIBOBS_API_PATCH_VER};

pub fn get_installed_version(obs_dll: &Path) -> anyhow::Result<Option<String>> {
    // The obs.dll should always exist
    let dll_exists = obs_dll.exists() && obs_dll.is_file();
    if !dll_exists {
        log::trace!("obs.dll does not exist at {}", obs_dll.display());
        return Ok(None);
    }

    log::trace!("Getting obs.dll version string");
    let version = unsafe { libobs::obs_get_version_string() };
    if version.is_null() {
        log::trace!("obs.dll does not have a version string");
        return Ok(None);
    }

    let version_str = unsafe { std::ffi::CStr::from_ptr(version) }.to_str();
    if version_str.is_err() {
        log::trace!(
            "obs.dll version string is not valid UTF-8: {}",
            version_str.err().unwrap()
        );
        return Ok(None);
    }

    return Ok(Some(version_str.unwrap().to_string()))
}

pub fn should_update(version_str: &str) -> anyhow::Result<bool> {
    let version = version_str.split('.').collect::<Vec<_>>();
    if version.len() != 3 {
        anyhow::bail!("Invalid version string: {}", version_str);
    }
    let major = version[0].parse::<u64>();
    let minor = version[1].parse::<u64>();
    let patch = version[2].parse::<u64>();

    if major.is_err() || minor.is_err() || patch.is_err() {
        anyhow::bail!("Invalid version string: {}", version_str);
    }

    let major = major.unwrap();
    let minor = minor.unwrap();
    let patch = patch.unwrap();

    return Ok(major != LIBOBS_API_MAJOR_VER as u64
        || minor != LIBOBS_API_MINOR_VER as u64
        || patch < LIBOBS_API_PATCH_VER as u64);
}
