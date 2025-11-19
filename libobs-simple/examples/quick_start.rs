use libobs_simple::quick_start::quick_start;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize OBS context with auto-download
    let _context = quick_start().await?;
    println!("OBS initialized successfully!");

    // Keep the application running for a bit to simulate usage
    std::thread::sleep(Duration::from_secs(2));

    println!("Done!");

    Ok(())
}
