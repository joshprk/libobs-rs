#!/bin/bash
echo "=== macOS Screen Recording Permission Test ==="
echo ""
echo "Running screen capture example..."
echo "If you see a system prompt asking for permission, click 'Allow'"
echo ""
echo "Current permissions status:"
echo "  1. Check System Settings → Privacy & Security → Screen Recording"
echo "  2. Look for 'macos_screen_capture' or your terminal"
echo ""

cd /Users/sulaiman/libobs-rs
./target/debug/examples/macos_screen_capture 2>&1 | grep -E "✓|✗|permission|Unable to get|completed"

echo ""
echo "Check output above for permission warnings"

