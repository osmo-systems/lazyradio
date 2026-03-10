# TUI Color Guidelines

This document describes the color conventions used in this application's TUI layer, inspired by the lazygit color scheme. Follow these rules consistently when adding new widgets, popups, or interactive elements.

---

## Palette

| Role | Color | Ratatui constant |
|---|---|---|
| Active borders / structural accent | Green | `Color::Green` |
| Selection background | Blue | `Color::Blue` |
| Selection foreground | White | `Color::White` |
| Key shortcut labels / interactive hints | Yellow | `Color::Yellow` |
| Section headers | Cyan | `Color::Cyan` |
| Status bar keys | Light cyan | `Color::LightCyan` |
| Status bar text | Cyan | `Color::Cyan` |
| Body text | White | `Color::White` |
| Dimmed / hints | Dark gray | `Color::DarkGray` |
| State: playing | Green | `Color::Green` |
| State: paused | Yellow | `Color::Yellow` |
| State: stopped | Gray | `Color::Gray` |
| State: loading | Cyan | `Color::Cyan` |
| State: error | Red | `Color::Red` |

---

## Rules

### Borders
All widgets use `Borders::ALL`. Border color signals whether a panel is **active/focused** or **passive**.

**Focused panels** (the user interacts with them): **Green** border, Green title.
This applies to: the station list and all popups.

The station list is special: it yields focus to any open popup. When a popup is visible the station list drops to the unfocused White style, even though no other background panel takes over.

```rust
// Derive border color once at the top of the draw function
let has_popup = app.help_popup
    || app.search_popup.is_some()
    || app.error_popup.is_some()
    || app.warning_popup.is_some();
let border_color = if has_popup { Color::White } else { Color::Green };

// Use it on every block that belongs to the station list
Block::default()
    .borders(Borders::ALL)
    .border_style(Style::default().fg(border_color))
```

**Passive panels** (display-only, never directly focused): **White** border, default title.
This applies to: Now Playing, Status Log.

```rust
Block::default()
    .borders(Borders::ALL)
    .border_style(Style::default().fg(Color::White))
    .title("Now Playing")
```

**Semantic exceptions** (override regardless of focus):
- **Error popups**: Red border (`Color::Red`)
- **Warning popups**: Yellow border (`Color::Yellow`)

### Selected / active list items
Highlighted rows use a **Blue** background with **White** foreground, bold. This matches lazygit's `selectedLineBgColor`.

```rust
// Selected
Style::default()
    .fg(Color::White)
    .bg(Color::Blue)
    .add_modifier(Modifier::BOLD)

// Unselected
Style::default().fg(Color::White)
```

### Active navigation tab
The currently active tab label uses **Green** foreground, bold — matching the border accent. Inactive tabs are plain White.

```rust
// Active
Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
// Inactive
Style::default().fg(Color::White)
```

### Key shortcut labels
Keys and shortcut labels throughout the UI use **Yellow**. This creates a clear visual distinction between "things you press" (Yellow) and "structural chrome" (Green).

```rust
Span::styled("Enter", Style::default().fg(Color::Yellow))
Span::styled("/", Style::default().fg(Color::Yellow))
```

### Interactive values (settings)
Setting values the user can cycle through follow the same pattern as key labels:
- Unselected: **Cyan** (visible but not focal)
- Selected: **Blue bg + White fg + Bold** (same as list selection)
- Selected label: **Green + Bold** (matching active tab accent)

```rust
// Unselected value
Style::default().fg(Color::Cyan)
// Selected value
Style::default().fg(Color::White).bg(Color::Blue).add_modifier(Modifier::BOLD)
// Selected label
Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
```

### Section headers inside popups
Group labels (e.g. "Navigation", "Playback") inside a popup use **Cyan** bold.

```rust
Span::styled("Navigation", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
```

### Body / description text
Plain descriptive text uses the terminal default or explicit White.

```rust
Span::raw("Navigate station list")
```

### Footer hints inside popups
Closing/navigation hints at the bottom of a popup use **DarkGray**.

```rust
Span::styled("Esc: close  Tab: switch tab", Style::default().fg(Color::DarkGray))
```

### Status bar
The bottom status bar uses two Cyan shades: `LightCyan` for key names, `Cyan` for their descriptions. This area has its own distinct color register, separate from the rest of the UI.

```rust
let key_style  = Style::default().fg(Color::LightCyan);
let text_style = Style::default().fg(Color::Cyan);
```

### Separator lines inside popups
Internal separator lines (`─` repeated) reuse the border accent: **Green**.

```rust
Span::styled("─".repeat(width), Style::default().fg(Color::Green))
```

### Semantic / state colors
These colors carry specific meaning and should not be repurposed:

| Color | Meaning |
|---|---|
| Green | Playing, valid, active (also structural accent) |
| Yellow | Paused, warning, key shortcuts, favorites star |
| Red | Error, stopped with error, invalid syntax |
| Gray | Stopped (neutral) |
| Cyan | Loading, info text, section headers |

### Syntax highlighting (search input)
- Valid field name: `Color::Green`
- Invalid field name: `Color::Red`
- Incomplete token: `Color::Yellow`

---

## Anti-patterns

- Do not introduce new accent colors (Blue, Magenta, etc.) as structural/border colors — Green is the single structural accent.
- Do not use `bg(Green)` for anything — Green is a foreground accent only. The selection background is Blue.
- Do not use `bg(Yellow)` — Yellow is always a foreground color (key labels, semantic states).
- Do not use Green for text that is not "active/selected" — Green as foreground text should only appear on active tabs, selected setting labels, and active borders.
- Do not use Cyan for interactive elements the user can trigger — Cyan is informational. Interactive triggers use Yellow.
