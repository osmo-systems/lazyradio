# Web Radio TUI

A terminal-based web radio browser and player built with Rust, featuring a rich TUI interface powered by [ratatui](https://github.com/ratatui-org/ratatui).

## Features

- 🎵 **Browse & Play Web Radio Stations** from the [Radio Browser](https://www.radio-browser.info/) API
- 🔍 **Multiple Browse Modes**: Popular stations, search by name, browse by country, genre, or language
- ⭐ **Favorites**: Save your favorite stations locally
- 📜 **History**: Track recently played stations
- 🎛️ **Player Controls**: Play, pause, stop, reload, volume control
- 📊 **Audio Visualizer**: Real-time audio visualization
- 👍 **Voting System**: Vote for your favorite stations on Radio Browser
- 💾 **Caching**: Smart caching to reduce API calls and speed up browsing
- 📝 **Logging**: Comprehensive logging with tracing
- 🖥️ **Cross-Platform**: Works on Linux, macOS, and Windows

## Screenshots

```
┌Tabs────────────────────────────────────────────────────────────────┐
│ Browse (1)  Favorites (2)  History (3)                             │
└────────────────────────────────────────────────────────────────────┘
┌Browse Mode─────────────────────────────────────────────────────────┐
│ Popular (F1)  Search (/)  Country (F2)  Genre (F3)  Language (F4)  │
└────────────────────────────────────────────────────────────────────┘
┌Stations (100 stations)─────────────────────────────────────────────┐
│ ♥ ● Jazz FM - USA - MP3 - 128 kbps                                 │
│   ● Classical Radio - UK - AAC - 192 kbps                           │
│   ● Rock Station - Germany - MP3 - 256 kbps                         │
└────────────────────────────────────────────────────────────────────┘
┌Player──────────────────────────────┬Visualizer──────────────────────┐
│ ▶ Playing                          │ ████                           │
│                                    │ █████                          │
│ Station: Jazz FM                   │ ████                           │
│ Volume:  ████████████░░░░░░░░ 60%  │ ███                            │
│                                    │ ████                           │
│ Controls: Enter=Play Space=Pause/  │ █████                          │
│          Resume S=Stop R=Reload   │ ████                           │
└────────────────────────────────────┴────────────────────────────────┘
┌Status──────────────────────────────────────────────────────────────┐
│ Keys: ↑/↓=Navigate Tab=Switch F=Favorite V=Vote Q=Quit             │
└────────────────────────────────────────────────────────────────────┘
```

## Installation

### Prerequisites

#### Linux
You need ALSA development libraries:

**Fedora/RHEL/CentOS:**
```bash
sudo dnf install alsa-lib-devel
```

**Debian/Ubuntu:**
```bash
sudo apt-get install libasound2-dev
```

**Arch Linux:**
```bash
sudo pacman -S alsa-lib
```

#### macOS
No additional dependencies required. Audio is handled through CoreAudio.

#### Windows
No additional dependencies required. Audio is handled through WASAPI.

### Building from Source

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone and build**:
   ```bash
   cd web-radio
   cargo build --release
   ```

3. **Run**:
   ```bash
   cargo run --release
   ```

Or install globally:
```bash
cargo install --path .
web-radio
```

## Usage

### Keyboard Shortcuts

#### Navigation
- `↑/↓` or `j/k`: Navigate through lists
- `PgUp/PgDn`: Page up/down
- `Tab`: Next tab
- `Shift+Tab`: Previous tab
- `1/2/3`: Quick switch to Browse/Favorites/History tabs

#### Browse Modes (in Browse tab)
- `F1` or `1`: Popular stations
- `F2` or `2`: Browse by country
- `F3` or `3`: Browse by genre
- `F4` or `4`: Browse by language
- `/`: Search by name

#### Playback
- `Enter`: Play selected station
- `Space`: Pause/Resume
- `S`: Stop playback
- `R`: Reload current station (reconnect)
- `+/=`: Volume up
- `-/_`: Volume down

#### Station Management
- `F`: Toggle favorite for selected station
- `V`: Vote for selected station (on Radio Browser API)

#### General
- `Esc`: Cancel search / Go back from browse list
- `Q`: Quit application

### Browse Modes

#### Popular Stations
Shows the most popular/voted stations from Radio Browser.

#### Search by Name
Press `/` to enter search mode, type your query, and press `Enter` to search.

#### Browse by Country/Genre/Language
1. Press `F2`, `F3`, or `F4` to enter browse mode
2. Select from the list using arrow keys
3. Press `Enter` to load stations
4. Press `Esc` to go back to the list

### Data Storage

All user data is stored in platform-specific directories:
- **Linux**: `~/.local/share/web-radio/`
- **macOS**: `~/Library/Application Support/web-radio/`
- **Windows**: `%APPDATA%\web-radio\`

Files stored:
- `favorites.toml`: Your favorite stations
- `history.toml`: Recently played stations
- `config.toml`: Application configuration
- `cache/`: Cached station lists
- `web-radio.log`: Application logs

### Configuration

Edit `config.toml` to customize:

```toml
# Cache duration in seconds (default: 3600 = 1 hour)
cache_duration_secs = 3600

# Maximum number of history entries (default: 50)
max_history_entries = 50

# Default volume (0.0 to 1.0, default: 0.5)
default_volume = 0.5

# Maximum number of stations to fetch per query (default: 100)
station_limit = 100
```

## Architecture

### Project Structure

```
src/
├── main.rs           # Entry point and event loop
├── app.rs            # Application state management
├── config.rs         # Configuration management
├── api/              # Radio Browser API client
│   ├── client.rs     # HTTP client with server discovery
│   └── models.rs     # Data models for stations
├── player/           # Audio playback
│   └── audio.rs      # Rodio-based audio player
├── storage/          # Data persistence
│   ├── favorites.rs  # Favorites management
│   ├── history.rs    # History tracking
│   └── cache.rs      # Station list caching
└── ui/               # Terminal UI
    └── layout.rs     # Ratatui-based UI layout
```

### Key Technologies

- **[ratatui](https://github.com/ratatui-org/ratatui)**: Terminal UI framework
- **[crossterm](https://github.com/crossterm-rs/crossterm)**: Cross-platform terminal manipulation
- **[rodio](https://github.com/RustAudio/rodio)**: Audio playback
- **[reqwest](https://github.com/seanmonstar/reqwest)**: HTTP client for API calls
- **[tokio](https://github.com/tokio-rs/tokio)**: Async runtime
- **[trust-dns-resolver](https://github.com/bluejekyll/trust-dns)**: DNS resolution for server discovery
- **[tracing](https://github.com/tokio-rs/tracing)**: Structured logging

## Radio Browser API

This application uses the free and open-source [Radio Browser API](https://www.radio-browser.info/). The API provides:
- Access to thousands of web radio stations
- Search and browse capabilities
- Station metadata (name, country, genre, bitrate, codec, etc.)
- Online status checking
- Click tracking and voting system

The application automatically discovers available API servers via DNS and load-balances requests across them.

## Troubleshooting

### Audio Issues

**No sound on Linux:**
- Ensure ALSA libraries are installed: `sudo dnf install alsa-lib-devel` (or equivalent for your distro)
- Check system volume and ensure audio output is not muted
- Verify audio works in other applications

**Crackling or stuttering:**
- Check your network connection stability
- Try a different station (some streams may have issues)
- Use the reload (R) command to reconnect

### Station Playback Issues

**"Failed to play station" error:**
- The station stream URL might be offline or changed
- Try another station
- Check logs in `~/.local/share/web-radio/web-radio.log`

**Station loads but no audio:**
- The codec might not be supported (though rodio supports most common formats)
- Try pressing `R` to reload the station
- Check if other stations work

### API Issues

**"No Radio Browser servers found":**
- Check your internet connection
- DNS resolution might be blocked (requires access to `all.api.radio-browser.info`)
- Check firewall settings

**Slow loading:**
- Initial load fetches station lists which can take a few seconds
- Subsequent browsing uses cached data (configurable via `cache_duration_secs`)

### Build Issues

**"alsa-sys" build fails:**
- Install ALSA development headers (see Installation Prerequisites)

**Other compilation errors:**
- Ensure you have the latest Rust: `rustup update`
- Try cleaning and rebuilding: `cargo clean && cargo build`

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for bugs and feature requests.

### Development

Run in development mode:
```bash
cargo run
```

Run with logging to terminal (for debugging):
```bash
RUST_LOG=debug cargo run
```

Run tests:
```bash
cargo test
```

## License

This project is open source. See LICENSE file for details.

## Acknowledgments

- [Radio Browser](https://www.radio-browser.info/) for providing the free radio station API
- [ratatui](https://github.com/ratatui-org/ratatui) for the excellent TUI framework
- [rodio](https://github.com/RustAudio/rodio) for cross-platform audio playback
- All the contributors to the Rust crates used in this project

## Future Enhancements

- [ ] Playlist support (queue multiple stations)
- [ ] Station recording
- [ ] Custom station URLs
- [ ] Themes/color schemes
- [ ] Mouse support
- [ ] Station metadata display (currently playing song, etc.)
- [ ] Equalizer
- [ ] Network radio protocols (e.g., Shoutcast directory)
- [ ] Last.fm scrobbling
