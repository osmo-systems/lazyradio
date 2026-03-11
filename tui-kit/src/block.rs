use ratatui::{
    text::{Line, Span},
    widgets::{Block, Borders},
};

use crate::Theme;

/// Creates a bordered [`Block`] for a main-content panel.
///
/// - `focused = true`  → uses [`Theme::border_focused`] (accent color, e.g. Green+Bold).
/// - `focused = false` → uses [`Theme::border_unfocused`] (e.g. White).
///
/// The title is any `Line<'static>` — use [`widget_title`] or [`crate::tabs::tab_line`]
/// to build it, or pass `Line::from("My Panel")` for a plain string.
pub fn panel_block(title: Line<'static>, focused: bool, theme: &Theme) -> Block<'static> {
    let border_style = if focused {
        theme.border_focused
    } else {
        theme.border_unfocused
    };
    Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(title)
}

/// Creates a bordered [`Block`] for a floating popup.
///
/// Always uses [`Theme::border_popup`] regardless of focus state,
/// since a visible popup is by definition the active element.
pub fn popup_block(title: Line<'static>, theme: &Theme) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(theme.border_popup)
        .title(title)
}

/// Builds a widget title [`Line`] with an optional keyboard-shortcut digit indicator.
///
/// The digit and `─` separator blend into the border line (same color), producing:
///
/// ```text
/// ┌─ 1 ─ Favorites ────────────────────────────────────────────┐
/// ```
///
/// - `shortcut = Some(1)` → ` [1] ─ Label `
/// - `shortcut = None`    → ` Label `
///
/// `active` controls the label style ([`Theme::tab_active`] vs [`Theme::tab_inactive`])
/// and the digit+separator color ([`Theme::border_focused`] vs [`Theme::border_unfocused`]),
/// so they always match the surrounding border.
///
/// # Example
///
/// ```ignore
/// // Simple panel title, always active
/// let title = widget_title("Now Playing", None, true, &theme);
///
/// // Panel with shortcut indicator, focus-aware
/// let title = widget_title("Favorites", Some(1), focused, &theme);
/// ```
pub fn widget_title(label: &str, shortcut: Option<u8>, active: bool, theme: &Theme) -> Line<'static> {
    let label_style = if active { theme.tab_active } else { theme.tab_inactive };
    let border_style = if active { theme.border_focused } else { theme.border_unfocused };

    match shortcut {
        Some(n) => Line::from(vec![
            Span::styled(format!("[{}]\u{2500} ", n), border_style),
            Span::styled(label.to_string(), label_style),
            Span::raw(" "),
        ]),
        None => Line::from(vec![
            Span::raw(" "),
            Span::styled(label.to_string(), label_style),
            Span::raw(" "),
        ]),
    }
}
