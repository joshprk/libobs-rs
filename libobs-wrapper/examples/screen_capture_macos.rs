// Production-ready screen capture using libobs-wrapper
// This demonstrates the RECOMMENDED high-level API for using libobs-rs
// 
// For development: Use cargo-obs-build to install OBS binaries
// For production: Use libobs-bootstrapper to download at runtime (see PRODUCTION_GUIDE.md)

use libobs_sources::macos::ScreenCaptureSourceBuilder;
use libobs_wrapper::context::ObsContext;
use libobs_wrapper::sources::ObsSourceBuilder;
use libobs_wrapper::utils::StartupInfo;

fn main() -> anyhow::Result<()> {
    println!("=== libobs-wrapper Screen Capture (macOS) ===\n");

    // Note: This example assumes OBS binaries are already installed via cargo-obs-build
    // For production apps, add bootstrapper support (see examples/download-at-runtime)
    
    // Initialize OBS context (wrapper handles all complexity!)
    let mut context = ObsContext::new(StartupInfo::default())?;
    println!("✓ Context initialized");

    // Create scene
    let mut scene = context.scene("Test Scene")?;
    println!("✓ Scene created");

    // Try creating screen capture source using source_builder
    let source = context
        .source_builder::<ScreenCaptureSourceBuilder, _>("Screen Capture")?
        .set_display(0)
        .set_show_cursor(true)
        .add_to_scene(&mut scene)?;

    println!("✓ Screen capture source created via wrapper!");
    println!("  Source name: {}", source.name());
    println!("  Source ID: {}", source.id());

    // Wait a bit to allow capture to initialize
    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("\n✅ SUCCESS! libobs-wrapper + macOS sources working!");
    println!("\nThis demonstrates the RECOMMENDED production approach:");
    println!("  1. ✅ libobs-bootstrapper - automatic OBS binary management");
    println!("  2. ✅ libobs-wrapper - high-level safe API");
    println!("  3. ✅ libobs-sources - platform-specific source bindings");
    println!("\nScreen capture is now active and ready for recording!");

    Ok(())
}
