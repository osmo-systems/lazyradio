use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use tui_kit::{
    block::{panel_block, popup_block, widget_title},
    popup::centered_popup,
    tabs::tab_line,
    Theme,
};

use crate::app::{App, HelpTab, Tab};
use rad_core::PlayerState;

pub fn draw(f: &mut Frame, app: &mut App) {
    let theme = Theme::default();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8), // Player + Status log section (at top)
            Constraint::Min(10),   // Main content (includes tabs in title)
            Constraint::Length(1), // Shortcuts bar (no border)
        ])
        .split(f.area());

    draw_player_and_log(f, app, chunks[0], &theme);
    draw_main_content(f, app, chunks[1], &theme);
    draw_status_bar(f, app, chunks[2], &theme);

    // Draw popups on top of everything
    if app.help_popup {
        draw_help_popup(f, app, &theme);
    }

    // Draw search popup
    if let Some(ref popup) = app.search_popup {
        popup.render(f, f.area(), &theme);
    }

    // Draw error popup on top of everything if present
    if app.error_popup.is_some() {
        draw_error_popup(f, app, &theme);
    }

    // Draw warning popup on top of everything if present
    if app.warning_popup.is_some() {
        draw_warning_popup(f, app, &theme);
    }
}

fn draw_main_content(f: &mut Frame, app: &mut App, area: Rect, theme: &Theme) {
    // Build tab titles
    let tab_title = tab_line(
        &[
            ("Browse", matches!(app.current_tab, Tab::Browse)),
            ("Favorites", matches!(app.current_tab, Tab::Favorites)),
            ("History", matches!(app.current_tab, Tab::History)),
        ],
        theme,
    );

    // All tabs just show station lists now
    draw_station_list(f, app, area, tab_title, theme);
}

fn draw_station_list(f: &mut Frame, app: &mut App, area: Rect, title: Line, theme: &Theme) {
    // Calculate visible stations count (area height minus borders and padding)
    // Each station takes 1 line, borders take 2 lines
    let visible_count = (area.height.saturating_sub(2)) as usize;
    let visible_count = visible_count.max(1);
    app.visible_stations_count = visible_count;

    // Keep the query limit in sync with the available rows.  On terminal resize this
    // triggers a re-fetch so the list always fills the screen exactly.
    if visible_count != app.current_query.limit {
        app.current_query.limit = visible_count;
        app.pages_cache.clear();
        app.pending_search = true;
    }

    // Station list loses focus when any popup is open
    let has_popup = app.help_popup
        || app.search_popup.is_some()
        || app.error_popup.is_some()
        || app.warning_popup.is_some();
    let border_style = if has_popup {
        theme.border_unfocused
    } else {
        theme.border_focused
    };

    if app.stations.is_empty() {
        let text = if app.loading {
            "Loading stations..."
        } else {
            "No stations found."
        };

        let paragraph = Paragraph::new(text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .border_style(border_style),
            )
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
        return;
    }

    let list_items: Vec<ListItem> = app
        .stations
        .iter()
        .enumerate()
        .map(|(i, station)| {
            let is_favorite = app.favorites.is_favorite(&station.station_uuid);
            let status_marker = if station.is_online() { "●" } else { "○" };

            let is_selected = i == app.selected_index;
            let is_voted = app.vote_manager.has_voted_recently(&station.station_uuid);
            let base_style = if is_selected {
                theme.selection
            } else if is_voted {
                Style::default().fg(Color::LightGreen)
            } else {
                Style::default().fg(Color::White)
            };

            // Build content with styled spans
            let mut spans = vec![];

            // Left margin (2 chars): yellow star for favorites, spaces otherwise
            if is_favorite {
                // Yellow star emoji for favorites (unless selected, then use selection colors)
                let star_style = if is_selected {
                    base_style
                } else {
                    Style::default().fg(Color::Yellow)
                };
                spans.push(Span::styled("⭐", star_style));
            } else {
                // Empty margin for non-favorites
                spans.push(Span::styled("  ", base_style));
            }

            // Status marker (online/offline)
            spans.push(Span::styled(status_marker, base_style));

            // Rest of the content
            let content_text = format!(
                " {} - {} - {} - {}",
                station.name,
                station.country,
                station.format_codec(),
                station.format_bitrate()
            );
            spans.push(Span::styled(content_text, base_style));

            ListItem::new(Line::from(spans))
        })
        .collect();

    // Combine title line with station count in a single Line
    let mut title_spans = title.spans;
    title_spans.push(Span::raw(" ("));
    title_spans.push(Span::styled(
        format!("{}", app.stations.len()),
        Style::default().fg(Color::Cyan),
    ));
    title_spans.push(Span::raw(" stations)"));
    let full_title = Line::from(title_spans);

    // Create pagination info for the right side if we have pagination
    let block = if app.current_page > 0 {
        let page_info = if app.is_last_page {
            format!("Page {}", app.current_page)
        } else {
            format!("Page {} →", app.current_page)
        };
        Block::default()
            .borders(Borders::ALL)
            .title(full_title)
            .title(
                ratatui::widgets::block::Title::from(Span::styled(page_info, border_style))
                    .alignment(Alignment::Right)
                    .position(ratatui::widgets::block::Position::Top),
            )
            .border_style(border_style)
    } else {
        Block::default()
            .borders(Borders::ALL)
            .title(full_title)
            .border_style(border_style)
    };

    let list = List::new(list_items).block(block);

    let mut state = ListState::default();
    state.select(Some(app.selected_index));

    f.render_stateful_widget(list, area, &mut state);
}

fn draw_player_and_log(f: &mut Frame, app: &App, area: Rect, theme: &Theme) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    draw_player(f, app, chunks[0], theme);
    draw_status_log(f, app, chunks[1], theme);
}

fn draw_status_log(f: &mut Frame, app: &App, area: Rect, theme: &Theme) {
    if app.status_log.is_empty() {
        let title = widget_title("Status Log", None, false, theme);
        let paragraph = Paragraph::new("No status messages yet")
            .block(panel_block(title, false, theme))
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(paragraph, area);
        return;
    }

    // Create list items from status log
    let list_items: Vec<ListItem> = app
        .status_log
        .iter()
        .map(|msg| ListItem::new(msg.as_str()).style(Style::default().fg(Color::White)))
        .collect();

    let title = widget_title("Status Log", None, false, theme);
    let list = List::new(list_items).block(panel_block(title, false, theme));

    let mut state = ListState::default();
    state.select(Some(app.status_log_scroll));

    f.render_stateful_widget(list, area, &mut state);
}

fn get_player_icon(state: PlayerState, frame: usize) -> &'static str {
    match state {
        PlayerState::Playing => {
            // Spinning vinyl record animation
            const PLAYING_ICONS: &[&str] = &["◐", "◓", "◑", "◒"];
            PLAYING_ICONS[frame % PLAYING_ICONS.len()]
        }
        PlayerState::Loading => {
            const LOADING_ICONS: &[&str] = &["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"];
            LOADING_ICONS[frame % LOADING_ICONS.len()]
        }
        PlayerState::Paused => "⏸",
        PlayerState::Stopped => "⏹",
        PlayerState::Error => "❌",
    }
}

fn draw_player(f: &mut Frame, app: &App, area: Rect, theme: &Theme) {
    let info = &app.player_info;

    // Get animated icon based on state
    let icon = get_player_icon(info.state, app.animation_frame);

    let state_name = match info.state {
        PlayerState::Playing => "Playing",
        PlayerState::Paused => "Paused",
        PlayerState::Stopped => "Stopped",
        PlayerState::Loading => "Loading...",
        PlayerState::Error => "Error",
    };

    let state_color = match info.state {
        PlayerState::Playing => Color::Green,
        PlayerState::Paused => Color::Yellow,
        PlayerState::Stopped => Color::Gray,
        PlayerState::Loading => Color::Cyan, // Changed from Blue for better visibility
        PlayerState::Error => Color::Red,
    };

    // Shorter volume bar (10 chars instead of 20)
    let volume_bar = {
        let filled = ((info.volume * 10.0).round()) as usize;
        let empty = 10 - filled;
        format!("{}{}", "█".repeat(filled), "░".repeat(empty))
    };

    // Show the currently playing station name prominently
    // Truncate station name if too long (calculate based on widget width)
    let max_station_length = area.width.saturating_sub(4) as usize; // Account for borders
    let station_display = if !info.station_name.is_empty() {
        let name = &info.station_name;
        if name.len() > max_station_length {
            format!("{}...", &name[..max_station_length.saturating_sub(3)])
        } else {
            name.clone()
        }
    } else {
        "No station selected".to_string()
    };

    // Calculate spacing for state and volume to be on the same line
    // We want: "[Icon] State" on left, "Vol: [bar] XX%" on right
    let volume_text = format!("Vol {} {:.0}%", volume_bar, (info.volume * 100.0).round());
    let state_text_len = icon.len() + 1 + state_name.len(); // icon + space + state name
    let volume_text_len = volume_text.len();
    let available_width = area.width.saturating_sub(4) as usize; // Account for borders and padding
    let spacing = if available_width > state_text_len + volume_text_len {
        available_width.saturating_sub(state_text_len + volume_text_len)
    } else {
        2 // Minimum spacing
    };

    let lines = vec![
        Line::from(""), // Empty line for spacing
        Line::from(vec![Span::styled(
            &station_display,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""), // Empty line for spacing
        Line::from(vec![
            Span::styled(icon, Style::default().fg(state_color)),
            Span::raw(" "),
            Span::styled(
                state_name,
                Style::default()
                    .fg(state_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" ".repeat(spacing)),
            Span::styled("Vol ", Style::default().fg(Color::Cyan)),
            Span::styled(&volume_bar, Style::default().fg(Color::Cyan)),
            Span::raw(format!(" {:.0}%", (info.volume * 100.0).round())),
        ]),
        Line::from(""), // Empty line for spacing
    ];

    let title = widget_title("Now Playing", None, false, theme);
    let block = panel_block(title, false, theme);

    let paragraph = Paragraph::new(lines)
        .block(block)
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect, theme: &Theme) {
    // Add horizontal margin to align with bordered widgets
    let margin_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

    let content_area = margin_chunks[1];

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(20)])
        .split(content_area);

    // Build contextual shortcuts depending on the active UI layer
    let mut pairs: Vec<(&str, &str)> = Vec::new();

    if app.error_popup.is_some() {
        pairs.push(("Esc/Enter", ":Close"));
        pairs.push(("Ctrl+C", ":Quit"));
    } else if app.warning_popup.is_some() {
        pairs.push(("Esc/Enter", ":Close"));
    } else if app.help_popup {
        match app.help_tab {
            HelpTab::Keys => {
                pairs.push(("Tab", ":Settings"));
                pairs.push(("Esc/?", ":Close"));
            }
            HelpTab::Settings => {
                pairs.push(("↑↓", ":Navigate"));
                pairs.push(("←→/Enter", ":Change"));
                pairs.push(("Tab", ":Keys"));
                pairs.push(("Esc", ":Close"));
            }
        }
    } else if app.search_popup.is_some() {
        pairs.push(("Enter", ":Search"));
        pairs.push(("Tab", ":Complete"));
        pairs.push(("↑↓", ":Suggestions"));
        pairs.push(("Esc", ":Cancel"));
    } else {
        // Main screen
        if app.stations.is_empty() {
            pairs.push(("/", ":Search"));
            pairs.push(("F1", ":Popular"));
        } else {
            pairs.push(("↑↓", ":Nav"));
            pairs.push(("Enter", ":Play"));

            match app.player_info.state {
                PlayerState::Playing | PlayerState::Paused => {
                    pairs.push(("Space", ":Pause"));
                    pairs.push(("s", ":Stop"));
                    pairs.push(("r", ":Reload"));
                }
                PlayerState::Loading => {
                    pairs.push(("s", ":Stop"));
                }
                _ => {}
            }

            pairs.push(("+-", ":Vol"));
            pairs.push(("f", ":Fav"));
            pairs.push(("/", ":Search"));
            pairs.push(("[]", ":Page"));
            pairs.push(("Tab", ":Tabs"));
        }
        pairs.push(("?", ":Help"));
        pairs.push(("Ctrl+C", ":Quit"));
    }

    let mut spans: Vec<Span> = Vec::new();
    for (i, (key, desc)) in pairs.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled(" | ", theme.hint));
        }
        spans.push(Span::styled(*key, theme.shortcut_key));
        spans.push(Span::styled(*desc, theme.hint));
    }

    let shortcuts = Paragraph::new(Line::from(spans)).alignment(Alignment::Left);
    f.render_widget(shortcuts, chunks[0]);

    // Version info on the right
    let version = env!("CARGO_PKG_VERSION");
    let version_line = Line::from(vec![
        Span::styled("rad ", theme.hint),
        Span::styled(version, theme.hint),
    ]);
    let version_widget = Paragraph::new(version_line).alignment(Alignment::Right);
    f.render_widget(version_widget, chunks[1]);
}

fn draw_error_popup(f: &mut Frame, app: &App, theme: &Theme) {
    if let Some(ref error_msg) = app.error_popup {
        // Calculate popup area (centered, 60% width, auto height based on content)
        let area = f.area();
        let popup_width = (area.width as f32 * 0.6).min(80.0) as u16;

        // Calculate height based on text content
        // Account for: borders (2), error message lines, empty line (1), footer (1)
        let content_width = popup_width.saturating_sub(4) as usize; // -4 for borders and padding

        // Estimate wrapped lines: count characters and divide by content width
        let estimated_lines = (error_msg.len() as f32 / content_width as f32).ceil() as u16;
        let popup_height = (estimated_lines + 2)
            .max(4)
            .min(area.height.saturating_sub(4));

        let popup_area = centered_popup(f, 0.6, 80, popup_height);

        // Create the error message with wrapping
        let error_text = vec![Line::from(Span::raw(error_msg))];

        let paragraph = Paragraph::new(error_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(theme.border_error)
                    .title(" Error ")
                    .title_style(theme.border_error),
            )
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Left);

        f.render_widget(paragraph, popup_area);
    }
}

fn draw_warning_popup(f: &mut Frame, app: &App, theme: &Theme) {
    if let Some(ref warning_msg) = app.warning_popup {
        let popup_area = centered_popup(f, 0.6, 80, 10);

        // Create the warning message with wrapping
        let warning_text = vec![
            Line::from(Span::styled(
                "Warning",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::raw(warning_msg)),
        ];

        let paragraph = Paragraph::new(warning_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(theme.border_warning)
                    .title(" Warning ")
                    .title_style(theme.border_warning),
            )
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Left);

        f.render_widget(paragraph, popup_area);
    }
}

fn draw_help_popup(f: &mut Frame, app: &App, theme: &Theme) {
    let popup_area = centered_popup(f, 0.7, 90, 26);

    // Tab titles in the border title — same pattern as the main station list tabs
    let title = tab_line(
        &[
            ("Keys", app.help_tab == HelpTab::Keys),
            ("Settings", app.help_tab == HelpTab::Settings),
        ],
        theme,
    );
    let block = popup_block(title, theme);
    f.render_widget(block, popup_area);

    // Content fills the interior directly (no inner tab bar or separator)
    let content_area = Rect {
        x: popup_area.x + 1,
        y: popup_area.y + 1,
        width: popup_area.width.saturating_sub(2),
        height: popup_area.height.saturating_sub(2),
    };

    match app.help_tab {
        HelpTab::Keys => draw_help_keys_content(f, content_area),
        HelpTab::Settings => draw_help_settings_content(f, app, content_area, theme),
    }
}

fn draw_help_keys_content(f: &mut Frame, area: Rect) {
    let help_text = vec![
        Line::from(vec![Span::styled(
            "Navigation",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("  ↑/↓ or j/k  ", Style::default().fg(Color::Yellow)),
            Span::raw("Navigate station list"),
        ]),
        Line::from(vec![
            Span::styled("  Tab / [ ]   ", Style::default().fg(Color::Yellow)),
            Span::raw("Switch between tabs (Browse/Favorites/History)"),
        ]),
        Line::from(vec![
            Span::styled("  1/2/3       ", Style::default().fg(Color::Yellow)),
            Span::raw("Jump to Browse/Favorites/History tab"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Playback",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("  Enter       ", Style::default().fg(Color::Yellow)),
            Span::raw("Play selected station"),
        ]),
        Line::from(vec![
            Span::styled("  Space       ", Style::default().fg(Color::Yellow)),
            Span::raw("Pause/Resume playback"),
        ]),
        Line::from(vec![
            Span::styled("  s           ", Style::default().fg(Color::Yellow)),
            Span::raw("Stop playback"),
        ]),
        Line::from(vec![
            Span::styled("  r           ", Style::default().fg(Color::Yellow)),
            Span::raw("Reload current station"),
        ]),
        Line::from(vec![
            Span::styled("  + / -       ", Style::default().fg(Color::Yellow)),
            Span::raw("Increase/Decrease volume"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Browse & Search",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("  F1          ", Style::default().fg(Color::Yellow)),
            Span::raw("Show popular stations"),
        ]),
        Line::from(vec![
            Span::styled("  /           ", Style::default().fg(Color::Yellow)),
            Span::raw("Search by name"),
        ]),
        Line::from(vec![
            Span::styled("  F2/F3/F4    ", Style::default().fg(Color::Yellow)),
            Span::raw("Browse by Country/Genre/Language"),
        ]),
        Line::from(vec![
            Span::styled("  f           ", Style::default().fg(Color::Yellow)),
            Span::raw("Toggle favorite on selected station"),
        ]),
        Line::from(vec![
            Span::styled("  v           ", Style::default().fg(Color::Yellow)),
            Span::raw("Vote for selected station"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Ctrl+C      ", Style::default().fg(Color::Yellow)),
            Span::raw("Quit application"),
        ]),
    ];

    f.render_widget(Paragraph::new(help_text).alignment(Alignment::Left), area);
}

fn draw_help_settings_content(f: &mut Frame, app: &App, area: Rect, theme: &Theme) {
    let label_width = 22usize;

    let settings: &[(&str, &str)] = &[
        ("Startup Tab", app.config.startup_tab.label()),
        (
            "Default Search Order",
            app.config.default_search_order.label(),
        ),
        (
            "Play at Startup",
            if app.config.play_at_startup {
                "On"
            } else {
                "Off"
            },
        ),
        (
            "Auto-vote Favorites",
            if app.config.auto_vote_favorites {
                "On"
            } else {
                "Off"
            },
        ),
    ];

    let mut lines = vec![Line::from("")];

    for (i, (label, value)) in settings.iter().enumerate() {
        let is_selected = i == app.settings_selected;
        let padding = label_width.saturating_sub(label.len());
        let label_part = format!("  {}{}", label, " ".repeat(padding));
        let value_part = format!("[ {} ]", value);

        let row = Line::from(vec![
            Span::styled(
                label_part,
                if is_selected {
                    theme.tab_active
                } else {
                    theme.body
                },
            ),
            Span::styled(
                value_part,
                if is_selected {
                    theme.selection
                } else {
                    theme.shortcut_key
                },
            ),
        ]);
        lines.push(row);
        lines.push(Line::from(""));
    }

    f.render_widget(Paragraph::new(lines).alignment(Alignment::Left), area);
}
