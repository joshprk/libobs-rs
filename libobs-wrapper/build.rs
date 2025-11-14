fn main() {
    // macOS: Set rpath for finding libobs.framework and dylibs
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path");
        println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path");
        println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path/..");
        println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path/..");

        // Copy helper binaries to examples directory
        copy_helper_binaries_macos();
    }
}

#[cfg(target_os = "macos")]
fn copy_helper_binaries_macos() {
    use std::fs;
    use std::path::PathBuf;

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());

    // Source: target/{profile}/obs-ffmpeg-mux
    // Dest: target/{profile}/examples/obs-ffmpeg-mux
    let target_dir = PathBuf::from(manifest_dir)
        .join("..")
        .join("target")
        .join(&profile);
    let helper_src = target_dir.join("obs-ffmpeg-mux");
    let examples_dir = target_dir.join("examples");
    let helper_dest = examples_dir.join("obs-ffmpeg-mux");

    if helper_src.exists() {
        // Create examples directory if it doesn't exist
        let _ = fs::create_dir_all(&examples_dir);

        // Copy helper binary
        if fs::copy(&helper_src, &helper_dest).is_ok() {
            println!("cargo:warning=Copied obs-ffmpeg-mux to examples directory");
        }
    }
}
