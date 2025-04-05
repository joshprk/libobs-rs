# libOBS Sources
This crate makes it really easy to create new sources for OBS Studio using the [libobs](https://crates.io/crates/libobs-new) crate.

## Example
```rust
// Set up the obs context and output here (see libobs-new crate for more info)

let (mut context, output) = initialize_obs(rec_file);
let output = context.get_output(&output).unwrap();

let windows =
    WindowCaptureSourceBuilder::get_windows(WindowSearchMode::ExcludeMinimized).unwrap();
let window = windows.into_iter().find(|w| {
    w.class
        .as_ref()
        .is_some_and(|e| e.to_lowercase().contains("notepad"))
})
.unwrap();

WindowCaptureSourceBuilder::new("test_capture")
    .set_window(&window)
    .add_to_output(output, 0)
    .unwrap();
// And the window capture source is added and captures the notepad window!
```