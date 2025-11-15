#[cfg(windows)]
mod windows;

#[cfg(windows)]
mod hdr_config;

fn main() {
    #[cfg(windows)]
    windows::main().unwrap();

    #[cfg(not(windows))]
    {
        eprintln!("This example is only supported on Windows.");
    }
}
