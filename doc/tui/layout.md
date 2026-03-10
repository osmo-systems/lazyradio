# TUI Layout Guidelines

This document describes the layout conventions used in this application's TUI layer. Follow these rules when adding new views, panels, or popups.

---

## Root layout

The screen is split into three vertical sections:

```
┌──────────────────────────────────────────────────────┐
│  Now Playing (50%)  │  Status Log (50%)              │  ← 8 lines (fixed)
├──────────────────────────────────────────────────────┤
│                                                      │
│              Main Content (tabs + list)              │  ← fills remaining space
│                                                      │
├──────────────────────────────────────────────────────┤
│  Shortcuts bar                                       │  ← 1 line (fixed, no border)
└──────────────────────────────────────────────────────┘
```

```rust
Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(8),  // player + log
        Constraint::Min(10),    // main content
        Constraint::Length(1),  // status bar
    ])
```

---

## Panel rules

### All bordered panels use `Borders::ALL`
Every content widget has a full border. The title appears at the top-left of the border.

```rust
Block::default()
    .borders(Borders::ALL)
    .border_style(Style::default().fg(Color::Yellow))
    .title(" Panel Title ")
```

### The status bar has no border
The bottom shortcuts bar is a single borderless line. A 1-char left/right margin is added manually to visually align it with the bordered panels above.

```rust
Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
        Constraint::Length(1),  // left margin
        Constraint::Min(0),     // content
        Constraint::Length(1),  // right margin
    ])
```

### Horizontal splits use percentage constraints
When splitting a section horizontally, prefer `Percentage` so it adapts to terminal width. Use `Length` only when a fixed column is clearly needed (e.g. a version string on the right).

---

## Popups

Popups float above the main content by rendering `Clear` on a centered `Rect` first, then drawing the widget on top.

### Centering a popup

```rust
let popup_width  = (area.width as f32 * 0.7).min(90.0) as u16;
let popup_height = 26u16;
let popup_x = (area.width.saturating_sub(popup_width)) / 2;
let popup_y = (area.height.saturating_sub(popup_height)) / 2;

let popup_area = Rect { x: popup_x, y: popup_y, width: popup_width, height: popup_height };

f.render_widget(Clear, popup_area);
// then render the actual widget on popup_area
```

### Width guidance
| Popup type | Recommended width |
|---|---|
| Help / settings | 70% of screen, max 90 cols |
| Error / warning | 60% of screen, max 80 cols |
| Search input | 80% of screen, max 100 cols |

### Popup with tabs
When a popup has multiple tabs, draw the outer border first as a plain `Block`, then manually compute inner `Rect`s for the tab bar, separator, and content area.

```
popup_area:
  y+0  ┌─ Title ──────────────────────────────┐
  y+1  │  Tab1   Tab2                         │  ← tab bar (1 line)
  y+2  │  ──────────────────────────────────  │  ← separator (Yellow ─)
  y+3  │  content ...                         │  ← content area
  ...  │                                      │
  y+H  └──────────────────────────────────────┘
```

Content area offset: `x+1, y+3, width-2, height-4`.

### Popup render order
Popups are rendered in priority order at the end of the draw function. Higher priority is rendered last (on top).

```
1. Help popup
2. Search popup
3. Error popup     ← highest priority, always on top
4. Warning popup
```

---

## Dynamic height

For popups whose content length is unknown at compile time (e.g. error messages that wrap), estimate the height from the text length:

```rust
let estimated_lines = (message.len() as f32 / content_width as f32).ceil() as u16;
let popup_height = (estimated_lines + 4).max(6).min(area.height.saturating_sub(4));
```

---

## Scrollable lists

Use `ListState` with `select()` to let ratatui handle scroll-into-view automatically. Store the selected index in the app state and pass it to `render_stateful_widget`.

```rust
let mut state = ListState::default();
state.select(Some(app.selected_index));
f.render_stateful_widget(list, area, &mut state);
```

Track visible row count from the widget area height minus border lines (2), and use it for page-up/page-down jump size:

```rust
let visible_count = (area.height.saturating_sub(2)) as usize;
app.visible_stations_count = visible_count.max(1);
```
