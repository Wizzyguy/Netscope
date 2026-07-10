use crate::{
    collector::dashboard_controls::ProcessRow,
    ui::{
        format::format_bytes,
        inspector::render_inspector,
        stats::DashboardStats,
    },
};

use ratatui::{
    prelude::*,
    widgets::*,
};

pub fn render_dashboard(
    frame: &mut Frame,
    search: &str,
    search_mode: bool,
    rows: &Vec<ProcessRow>,
    selected: usize,
) {
    let stats = DashboardStats::from_rows(rows, search);

    //--------------------------------------------------
    // Main Layout
    //--------------------------------------------------

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(9),
            Constraint::Length(2),
        ])
        .split(frame.area());

    //--------------------------------------------------
    // Header
    //--------------------------------------------------

    let header = Paragraph::new("NetScope")
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title("NetScope")
                .borders(Borders::ALL),
        );

    frame.render_widget(header, layout[0]);

    //--------------------------------------------------
    // Body
    //--------------------------------------------------

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(72),
            Constraint::Percentage(28),
        ])
        .split(layout[1]);

    //--------------------------------------------------
    // Left Side
    //--------------------------------------------------

    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
        ])
        .split(body[0]);

    //--------------------------------------------------
    // Search Box
    //--------------------------------------------------

    let search_text = if search_mode {
        format!("Search: {}_", search)
    } else if search.is_empty() {
        "Search: None".to_string()
    } else {
        format!("Search: {}", search)
    };

    frame.render_widget(
        Paragraph::new(search_text)
            .block(
                Block::default()
                    .title("Filter")
                    .borders(Borders::ALL),
            ),
        left[0],
    );

    //--------------------------------------------------
    // Process Table
    //--------------------------------------------------

    let header = Row::new(vec![
        "PID",
        "Process",
        "RX Total",
        "TX Total",
        "RX/s",
        "TX/s",
    ])
    .style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    );

    let table_rows = rows.iter().map(
        |(pid, name, rx, tx, rx_speed, tx_speed)| {
            Row::new(vec![
                pid.to_string(),
                name.clone(),
                format_bytes(*rx),
                format_bytes(*tx),
                format!("{}/s", format_bytes(*rx_speed)),
                format!("{}/s", format_bytes(*tx_speed)),
            ])
        },
    );

    let table = Table::new(
        table_rows,
        [
            Constraint::Length(7),
            Constraint::Percentage(38),
            Constraint::Length(14),
            Constraint::Length(14),
            Constraint::Length(12),
            Constraint::Length(12),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title("Processes")
            .borders(Borders::ALL),
    )
    .row_highlight_style(
        Style::default()
            .bg(Color::Blue)
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    )
    .highlight_symbol("▶ ");

    let mut state = TableState::default();

    if !rows.is_empty() {
        state.select(Some(selected));
    }

    frame.render_stateful_widget(
        table,
        left[1],
        &mut state,
    );

    //--------------------------------------------------
    // Statistics Panel
    //--------------------------------------------------

    let stats_panel = Paragraph::new(vec![
        Line::from(format!(
            "Processes : {}",
            stats.total_processes
        )),
        Line::from(format!(
            "Active    : {}",
            stats.active_processes
        )),
        Line::from(""),
        Line::from(format!(
            "Download  : {}",
            stats.rx_string()
        )),
        Line::from(format!(
            "Upload    : {}",
            stats.tx_string()
        )),
        Line::from(""),
        Line::from(format!(
            "Search    : {}",
            stats.search
        )),
    ])
    .block(
        Block::default()
            .title("Statistics")
            .borders(Borders::ALL),
    );

    frame.render_widget(
        stats_panel,
        body[1],
    );

    //--------------------------------------------------
    // Inspector
    //--------------------------------------------------

    if let Some((
        pid,
        process,
        rx,
        tx,
        rx_speed,
        tx_speed,
    )) = rows.get(selected)
    {
        render_inspector(
            frame,
            layout[2],
            *pid,
            process,
            &format_bytes(*rx),
            &format_bytes(*tx),
            &format!("{}/s", format_bytes(*rx_speed)),
            &format!("{}/s", format_bytes(*tx_speed)),
        );
    } else {
        frame.render_widget(
            Paragraph::new("No process selected")
                .block(
                    Block::default()
                        .title("Inspector")
                        .borders(Borders::ALL),
                ),
            layout[2],
        );
    }

    //--------------------------------------------------
    // Footer
    //--------------------------------------------------

    let footer = Paragraph::new(
        "↑↓ Move   / Search   D Download   U Upload   N Name   P PID   q Quit",
    )
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL),
    );

    frame.render_widget(
        footer,
        layout[3],
    );
}
