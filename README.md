# rad

Terminal radio player powered by [Radio Browser](https://www.radio-browser.info/).

## Installation

### Homebrew (macOS)

```bash
brew install osmo-systems/tap/rad
```

### From source

**Linux** requires ALSA dev libraries:
```bash
# Debian/Ubuntu
sudo apt-get install libasound2-dev
# Fedora
sudo dnf install alsa-lib-devel
# Arch
sudo pacman -S alsa-lib
```

```bash
git clone https://github.com/osmo-systems/radm.git
cd radm
cargo install --path rad-tui
```

## Usage

```
rad          # launch TUI
rad --help   # CLI usage
```

### Key bindings

| Key | Action |
|-----|--------|
| `/` | Search |
| `Enter` | Play selected station |
| `Space` | Pause / Resume |
| `S` | Stop |
| `R` | Reload stream |
| `F` | Toggle favorite |
| `V` | Vote on Radio Browser |
| `[` / `]` | Previous / Next page |
| `Tab` | Cycle focus |
| `1` / `2` / `3` | Jump to Stations / Log / Autovote |
| `+` / `-` | Volume up / down |
| `Ctrl+C` | Quit |

### Search syntax

Plain text searches by station name. Field queries allow precise filtering:

```
jazz
name=BBC country=UK
tag=classical bitrate_min=192 order=votes reverse=true
```

Available fields: `name`, `country`, `countrycode`, `state`, `language`, `tag`, `codec`, `bitrate_min`, `bitrate_max`, `order`, `reverse`, `hidebroken`, `is_https`, `page`.

### Shell completion

```bash
rad completion >> ~/.zshrc   # zsh
```

## Configuration

Config file: `~/Library/Application Support/radm/config.toml` (macOS) or `~/.config/radm/config.toml` (Linux).

```toml
cache_duration_secs = 3600
max_history_entries = 50
default_volume = 0.5
station_limit = 100
```

## License

Licensed under either [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
