# Web Radio - Troubleshooting Guide

## Issue: "Nothing happens when pressing Enter"

### Root Cause Analysis

There are several possible reasons why pressing Enter doesn't play a station:

### 1. **No Stations Loaded** (Most Common)

**Symptoms:**
- The station list area shows "No stations loaded"
- Pressing Enter has no effect
- No error popup appears

**Why this happens:**
- Network connectivity issues preventing API access
- Initial load failed silently
- You're in the wrong tab (Favorites/History with no items)

**Solution:**
- Press **F1** to manually load popular stations
- Check the log file: `~/.local/share/web-radio/web-radio.log.*`
- Look for error messages like "Failed to load popular stations"
- Verify internet connection and DNS resolution

**What you should see when stations ARE loaded:**
```
┌Browse (1)───Favorites (2)───History (3)─────────────┐
│ Browse (50 stations)                                │
│  ● Station Name - Country - MP3 - 128 kbps         │ <- Yellow highlight
│  ● Another Station - Country - AAC - 64 kbps       │
│  ● Third Station - Country - MP3 - 192 kbps        │
└─────────────────────────────────────────────────────┘
```

### 2. **Audio Device Not Available**

**Symptoms:**
- Error popup appears: "Failed to initialize audio device"
- Stations are loaded but playback doesn't start

**Why this happens:**
- No audio output device (headless system)
- Audio device in use by another application
- ALSA/PulseAudio/audio driver issues on Linux

**Solution:**
- Check audio device: `aplay -l` (Linux)
- Test audio: `speaker-test -c 2` (Linux)
- Close other audio applications
- Check system audio settings

### 3. **Player State Issues**

**Symptoms:**
- Player shows "Loading..." indefinitely
- Player shows "Error" state
- Error popup with playback error message

**Why this happens:**
- Station URL is invalid or playlist can't be parsed
- Network timeout during stream connection
- Unsupported audio format

**Solution:**
- Press 'S' to stop, then try another station
- Check logs for specific error messages
- Try stations with different codecs (MP3 vs AAC)

### 4. **UI Not in Expected State**

**Symptoms:**
- Browse list mode is active (showing countries/genres/languages)
- Search mode is active (cursor blinking)
- Error popup is open

**Solution:**
- Press **Esc** to exit browse/search/error modes
- Return to main station list
- Try Enter again

## Diagnostic Checklist

Run through this checklist to diagnose the issue:

1. **Check if running in proper terminal:**
   ```bash
   tty
   # Should output something like /dev/pts/0, NOT "not a tty"
   ```

2. **Check network connectivity:**
   ```bash
   ping -c 1 all.api.radio-browser.info
   ```

3. **Check audio device:**
   ```bash
   # Linux
   aplay -l
   
   # macOS
   system_profiler SPAudioDataType
   ```

4. **Check application logs:**
   ```bash
   tail -f ~/.local/share/web-radio/web-radio.log.*
   ```

5. **Run with verbose output:**
   ```bash
   RUST_LOG=debug ./target/release/web-radio
   ```

## Expected Behavior

### When App Starts Successfully:

1. **Initial screen shows:**
   - Browse tab active
   - 50 popular stations loaded
   - Player in "Stopped" state
   - Status bar with keyboard shortcuts

2. **When you press ↑/↓:**
   - Yellow highlight moves between stations
   - Selection is visible immediately

3. **When you press Enter:**
   - Status bar shows: "Playing: [Station Name]"
   - Player state changes to "Loading..."
   - Then changes to "Playing" (▶)
   - Station name appears in player section
   - After a few seconds, you should hear audio

4. **Visual feedback timeline:**
   ```
   0s:   Press Enter
   0.1s: Status message appears
   0.2s: Player shows "Loading..."
   1-3s: Player shows "Playing"
   2-5s: Audio starts (depending on buffering)
   ```

## Key Bindings Reference

| Key | Action |
|-----|--------|
| **F1** | Load Popular Stations |
| **/** | Search by name |
| **Enter** | Play selected station |
| **Space** | Pause/Resume |
| **S** | Stop |
| **R** | Reload current station |
| **+/-** | Volume up/down |
| **F** | Toggle favorite |
| **V** | Vote for station |
| **↑/↓** | Navigate |
| **Tab** | Switch tabs |
| **Q** | Quit |
| **Esc** | Close popup/Cancel |

## Log File Locations

- **Linux:** `~/.local/share/web-radio/web-radio.log.*`
- **macOS:** `~/Library/Application Support/web-radio/web-radio.log.*`
- **Windows:** `%APPDATA%\web-radio\web-radio.log.*`

## What to Look For in Logs

### Successful startup:
```
INFO web_radio: Starting Web Radio TUI
INFO web_radio::api::client: Initializing Radio Browser API client
INFO web_radio::api::client: Found 1 Radio Browser servers
INFO web_radio: API client initialized
INFO web_radio: Audio player initialized
INFO web_radio: App initialized
INFO web_radio: Initial data loaded. Stations count: 50
```

### Failed network:
```
ERROR web_radio: Failed to load popular stations: Failed to resolve Radio Browser DNS
```

### Failed audio:
```
ERROR Failed to initialize audio output stream: ...
```

### Successful playback:
```
INFO web_radio: play_selected called, stations count: 50, selected_index: 0
INFO web_radio: Playing station: Radio Station Name - URL: http://...
INFO web_radio::player::audio: Starting playback: Radio Station Name - http://...
INFO web_radio::player::audio: Buffered 524288 bytes, starting playback
INFO web_radio::player::audio: Audio playback started
```

## Still Having Issues?

If you've tried everything above and still have issues:

1. **Collect diagnostic info:**
   ```bash
   # System info
   uname -a
   
   # Audio devices
   aplay -l  # Linux
   
   # Network test
   curl -I https://all.api.radio-browser.info
   
   # App logs
   cat ~/.local/share/web-radio/web-radio.log.* | tail -100
   ```

2. **Try a minimal test:**
   - Start the app
   - Press F1 (load popular)
   - Wait 5 seconds
   - Press Enter
   - Wait 10 seconds
   - Check what you see and hear

3. **Common environment issues:**
   - **Docker/Container:** No audio device available
   - **SSH/Remote:** No TTY or audio device
   - **WSL:** Audio requires WSLg or PulseAudio setup
   - **Wayland:** May need XWayland for audio
   - **Flatpak/Snap:** Permissions for audio and network

## Quick Test Command

```bash
# This should work on a system with audio and network:
./target/release/web-radio

# Then:
# 1. Wait 2 seconds for stations to load
# 2. Press Enter to play first station
# 3. Wait 5 seconds for audio to start
# 4. Press Q to quit
```

If this doesn't work, check the logs and follow the diagnostic checklist above.
