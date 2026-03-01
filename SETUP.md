# Quick Setup Guide

## Prerequisites

Before building, you need to install system audio libraries.

### Linux (Your System)

You're on a **Fedora/RHEL-based system**, so install ALSA development libraries:

```bash
sudo dnf install alsa-lib-devel
```

### Other Platforms

**macOS**: No additional dependencies needed
**Windows**: No additional dependencies needed
**Debian/Ubuntu**: `sudo apt-get install libasound2-dev`
**Arch Linux**: `sudo pacman -S alsa-lib`

## Building

Once ALSA libraries are installed:

```bash
# Build the project
cargo build --release

# Run the application
cargo run --release
```

## Alternative: Use the Install Script

```bash
./install.sh
```

The script will:
1. Check if Rust is installed (install if needed)
2. Check for system dependencies (install if possible)
3. Build the application

## Quick Start

Once running:

1. **Browse Popular Stations**: Press `F1` or just wait for initial load
2. **Navigate**: Use `↑/↓` arrow keys
3. **Play a Station**: Press `Enter` on selected station
4. **Control Playback**: 
   - `Space`: Pause/Resume
   - `+/-`: Volume up/down
   - `R`: Reload station
   - `S`: Stop
5. **Save Favorites**: Press `F` on any station
6. **Search**: Press `/`, type query, press `Enter`
7. **Browse by Country/Genre**: Press `F2` or `F3`, select, press `Enter`
8. **Quit**: Press `Q`

## Troubleshooting

**Build fails with "alsa-sys" error:**
- You need to install ALSA development headers first (see above)

**No sound:**
- Check system volume
- Try a different station
- Check logs: `~/.local/share/web-radio/web-radio.log`

**Slow initial load:**
- First launch fetches station lists from API (normal)
- Subsequent launches use cached data

## File Locations

- **Config**: `~/.local/share/web-radio/config.toml`
- **Favorites**: `~/.local/share/web-radio/favorites.toml`
- **History**: `~/.local/share/web-radio/history.toml`
- **Cache**: `~/.local/share/web-radio/cache/`
- **Logs**: `~/.local/share/web-radio/web-radio.log`
