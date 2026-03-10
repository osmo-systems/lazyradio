# TUI Navigation Guidelines

This document describes the navigation conventions used in this application's TUI layer. Follow these rules to keep keyboard interaction consistent across all screens and popups.

---

## Layered input model

Key events are consumed by the topmost active layer. Processing stops as soon as a layer handles the event (`return` after handling). Layers in priority order (highest first):

1. **Ctrl+C** — always quits, checked before anything else
2. **Help popup** (Keys tab or Settings tab)
3. **Error / Warning popup**
4. **Search popup**
5. **Main screen**

Each layer is an `if` guard at the top of `handle_key_event`. If a layer is active, it handles the key and returns early, so lower layers never see it.

```rust
// Always first
if modifiers.contains(KeyModifiers::CONTROL) && key == KeyCode::Char('c') {
    app.quit(); return;
}

// Popups in priority order
if app.help_popup   { /* ... */ return; }
if app.error_popup.is_some() { /* ... */ return; }
if app.search_popup.is_some() { /* ... */ return; }

// Main screen — reached only when nothing is open
match key { ... }
```

---

## Popup conventions

### Opening a popup
Each popup is opened by a dedicated key from the main screen. The key should be memorable and ideally match common TUI conventions.

| Popup | Key | Notes |
|---|---|---|
| Help | `?` | Universal TUI convention |
| Search | `/` | Vi-style search |

When opening a popup, reset its internal state (selected tab, cursor position, etc.) so it always opens in a clean default state.

```rust
KeyCode::Char('?') => {
    app.help_popup = true;
    app.help_tab = HelpTab::Keys;   // reset to first tab
    app.settings_selected = 0;       // reset selection
}
```

### Closing a popup
All popups are closed with `Esc`. The `?` key toggles the help popup (also closes it). Never require the user to navigate to a "Close" button.

```rust
KeyCode::Esc | KeyCode::Char('?') => { app.help_popup = false; }
KeyCode::Esc | KeyCode::Enter     => { app.close_error_popup(); }
```

When closing an error popup caused by a daemon/background state, also clear that underlying state so the popup does not immediately reopen on the next tick.

```rust
KeyCode::Esc | KeyCode::Enter => {
    app.close_error_popup();
    let _ = daemon_conn.clear_error().await;
    app.player_info.error_message = None; // prevent same-tick re-trigger
}
```

---

## Tab navigation

### Main tabs
Switch between the top-level tabs (Browse / Favorites / History) with:

| Key | Action |
|---|---|
| `Tab` | Next tab (wraps around) |
| `Shift+Tab` / `BackTab` | Previous tab |
| `1` / `2` / `3` | Jump directly to tab by position |

Tabs wrap around at both ends.

### Popup tabs
When a popup has multiple tabs (e.g. Help: Keys / Settings), use `Tab` to cycle forward through them. There is no back-tab inside a popup — the number of tabs is kept small enough that cycling forward is sufficient.

```rust
KeyCode::Tab => { app.help_tab = HelpTab::Settings; }
```

---

## List navigation

Standard movement keys apply to any scrollable list (station list, browse lists, autocomplete):

| Key | Action |
|---|---|
| `↑` / `k` | Move selection up by 1 |
| `↓` / `j` | Move selection down by 1 |
| `PageUp` | Move up by one visible page |
| `PageDown` | Move down by one visible page |
| `Home` | Jump to first item |
| `End` | Jump to last item |

Provide both arrow keys and vi-style (`j`/`k`) for every list. Page size should match the number of currently visible rows.

---

## Settings / option cycling

When a setting has a fixed set of options, the user cycles through them with directional keys:

| Key | Action |
|---|---|
| `→` / `Enter` | Next option (cycle forward) |
| `←` | Previous option (cycle backward) |
| `↑` / `k` | Move to previous setting row |
| `↓` / `j` | Move to next setting row |

Changes take effect immediately and are persisted on every keystroke (no explicit Save action). This matches the "live preview" expectation users have for TUI settings panels.

```rust
KeyCode::Right | KeyCode::Enter => {
    app.config.my_setting = app.config.my_setting.cycle_next();
    let _ = app.config.save(&app.data_dir);
}
```

---

## Autocomplete / inline suggestions

When a text input has autocomplete:

| Key | Action |
|---|---|
| `Tab` | Accept current suggestion |
| `↓` / `↑` | Navigate suggestion list |
| `Esc` (first press) | Close suggestion list, keep input |
| `Esc` (second press) | Close the entire input popup |

The two-stage Esc gives the user a way to dismiss suggestions without losing their typed input.

---

## Action keys (main screen)

Prefer single-letter keys for frequent actions. Support both lower and upper case.

| Category | Keys | Rule |
|---|---|---|
| Playback | `Enter`, `Space`, `s`, `r` | `Enter` = play, `Space` = pause/resume toggle, `s` = stop, `r` = reload |
| Volume | `+`/`=`, `-`/`_` | Accept both shifted and unshifted variants of the same physical key |
| Favorites | `f` | Toggle |
| Search / browse | `/`, `F1`–`F4` | `/` for text search, Fn for mode switches |
| Pagination | `[`, `]` | Previous / next page — bracket keys suggest "moving through pages" |
| Quit | `Ctrl+C`, `q`/`Q` | `Ctrl+C` is the primary; `q` provided as convenience |

Always match a main-screen key against all active modifiers before falling through to the generic `match` block.

---

## Deferred actions

Some actions (search, page change) cannot be executed synchronously inside the event handler because they are `async` and need `await`. Use a flag in the app state and execute on the next main loop iteration.

```rust
// In key handler (sync context):
app.pending_search = true;
app.pending_page_change = Some(1);

// In main loop (async context):
if app.pending_search {
    app.pending_search = false;
    app.execute_search().await?;
}
```

This keeps `handle_key_event` a plain `async fn` that only mutates app state, while heavy I/O runs in the main loop where it can be properly awaited.
