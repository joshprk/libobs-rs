# Testing Cross-Compiled Windows Binary

## What Was Built

Cross-compiled from macOS to Windows x64 using MinGW-w64.

**Binary:** `target/x86_64-pc-windows-gnu/debug/record.exe` (116MB)
**Platform:** Windows x64 (GNU ABI)
**OBS Version:** 32.0.2

## Files to Copy to Windows

Copy the **entire** `target/x86_64-pc-windows-gnu/debug/` directory to your Windows machine.

This includes:
- `record.exe` - The recording application
- `obs.dll` - OBS library
- `obs-plugins/64bit/*.dll` - All OBS plugins
- `data/` - OBS data files (effects, locales, etc.)

## How to Test on Windows

### Option 1: Copy Entire Directory
```cmd
REM On Windows machine, navigate to the copied directory
cd target\x86_64-pc-windows-gnu\debug
record.exe
```

### Option 2: Package as ZIP
```bash
# On macOS, create a zip
cd target/x86_64-pc-windows-gnu/debug
zip -r ~/Desktop/libobs-windows-test.zip record.exe obs.dll obs-plugins/ data/

# Transfer ~/Desktop/libobs-windows-test.zip to Windows
# Extract and run record.exe
```

## Expected Behavior

When you run `record.exe` on Windows:

1. ✅ Shows "=== Bootstrapper + Recording Test ==="
2. ✅ Shows "✓ OBS ready" (uses downloaded binaries)
3. ✅ Shows "✓ Context initialized"
4. ✅ Shows "✓ Windows monitor capture ready"
5. ✅ Shows "✓ Encoders configured"
6. ✅ Shows "🔴 Starting recording..."
7. ✅ Shows "✓ Recording started!"
8. ⏳ Waits 5 seconds
9. ✅ Shows "⏹️ Recording stopped"
10. ✅ Shows file size in bytes
11. ✅ Creates: `%USERPROFILE%\Desktop\bootstrapper_recording.mp4`

## What This Proves

If successful, this proves:
- ✅ Cross-compilation macOS → Windows works
- ✅ Platform detection downloads correct binaries
- ✅ libobs-wrapper works on Windows
- ✅ libobs-sources Windows monitor capture works
- ✅ libobs-bootstrapper Windows support works
- ✅ Complete recording pipeline works cross-platform

## Troubleshooting

### "obs.dll not found"
- Ensure you copied the entire debug directory
- Run from within the debug directory

### "Module not loaded" errors
- Ensure `obs-plugins/64bit/` directory exists
- Check `data/` directory exists

### Recording fails
- Windows may need different encoder settings
- Check console output for specific error messages

## Success Criteria

✅ Binary runs without DLL errors
✅ Context initializes
✅ Monitor capture source creates
✅ Recording starts
✅ MP4 file created on Desktop
✅ Video file is > 0 bytes and plays

If all criteria met: **Cross-compilation fully works!** 🎉

