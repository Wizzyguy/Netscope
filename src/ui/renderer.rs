use crate::ui::{
    format::format_bytes,
    stats::DashboardStats,
};

use ratatui::{
    prelude::*,
    widgets::*,
};

pub fn render_dashboard(
    frame: &mut Frame,
    search: &str,
    search_mode: bool,
    rows: &Vec<(u32, String, u64, u64)>,
) {
    let stats = DashboardStats::from_rows(rows, search);

    // Main layout
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(10),   // Body
            Constraint::Length(2), // Footer
        ])
        .split(frame.area());

    //----------------------------------------
    // Header
    //----------------------------------------

    let header = Paragraph::new("NetScope")
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title("NetScope")
                .borders(Borders::ALL),
        );

    frame.render_widget(header, layout[0]);

    //----------------------------------------
    // Split body
    //----------------------------------------

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(72),
            Constraint::Percentage(28),
        ])
        .split(layout[1]);

    //----------------------------------------
    // Left panel
    //----------------------------------------

    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
        ])
        .split(body[0]);

    //----------------------------------------
    // Search
    //----------------------------------------

    let search_text = if search_mode {
        format!("Search: {}_", search)
    } else if search.is_empty() {
        "Search: None".to_string()
    } else {
        format!("Search: {}", search)
    };

    let search_widget = Paragraph::new(search_text)
        .block(
            Block::default()
                .title("Filter")
                .borders(Borders::ALL),
        );

    frame.render_widget(search_widget, left[0]);

    //----------------------------------------
    // Process Table
    //----------------------------------------

    let header = Row::new(vec![
        "PID",
        "Process",
        "RX",
        "TX",
    ])
    .style(
        Style::default()
            .add_modifier(Modifier::BOLD),
    );

    let table_rows = rows.iter().map(|(pid, name, rx, tx)| {
        Row::new(vec![
            pid.to_string(),
            name.clone(),
            format_bytes(*rx),
            format_bytes(*tx),
        ])
    });

    let table = Table::new(
        table_rows,
        [
            Constraint::Length(8),
            Constraint::Percentage(45),
            Constraint::Length(14),
            Constraint::Length(14),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title("Processes")
            .borders(Borders::ALL),
    );

    frame.render_widget(table, left[1]);

    //----------------------------------------
    // Statistics Panel
    //----------------------------------------

    let stats_text = vec![
        Line::from(format!("Processes : {}", stats.total_processes)),
        Line::from(format!("Active    : {}", stats.active_processes)),
        Line::from(""),
        Line::from(format!("Download  : {}", stats.rx_string())),
        Line::from(format!("Upload    : {}", stats.tx_string())),
        Line::from(""),
        Line::from(format!("Search    : {}", stats.search)),
    ];

    let stats_panel = Paragraph::new(stats_text)
        .block(
            Block::default()
                .title("Statistics")
                .borders(Borders::ALL),
        );

    frame.render_widget(stats_panel, body[1]);

    //----------------------------------------
    // Footer
    //----------------------------------------

    let footer = Paragraph::new(
        " q Quit   / Search   Esc Cancel   Enter Apply ",
    )
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL),
    );

    frame.render_widget(footer, layout[2]);
}
