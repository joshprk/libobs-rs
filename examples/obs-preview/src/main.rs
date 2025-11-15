#[cfg(windows)]
mod windows;

fn main() {
    #[cfg(windows)]
    windows::main();

    #[cfg(not(windows))]
    {
        eprintln!("This example is only supported on Windows.");
    }
}
