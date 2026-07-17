use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    widgets::*,
};

use crate::{
    collector::ProcessSession,
    ui::{
        format::format_bytes,
        inspector::render_inspector,
        stats::DashboardStats,
        App,
    },
};

pub fn render_dashboard_view(
    frame: &mut Frame,
    area: Rect,
    app: &App,
    rows: &Vec<ProcessSession>,
) {
    //----------------------------------------------------------
    // Layout
    //----------------------------------------------------------

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(12),
        ])
        .split(area);

    //----------------------------------------------------------
    // Dashboard Cards
    //----------------------------------------------------------

    let cards = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(layout[0]);

    draw_card(
        frame,
        cards[0],
        "Download",
        format_bytes(app.total_download),
    );

    draw_card(
        frame,
        cards[1],
        "Upload",
        format_bytes(app.total_upload),
    );

    draw_card(
        frame,
        cards[2],
        "Total Usage",
        format_bytes(app.total_bandwidth()),
    );

    draw_card(
        frame,
        cards[3],
        "Processes",
        rows.len().to_string(),
    );

    //----------------------------------------------------------
    // Search
    //----------------------------------------------------------

    let search_text = if app.search_mode {
        format!("Search: {}_", app.search)
    } else if app.search.is_empty() {
        "Press '/' to search".into()
    } else {
        format!("Search: {}", app.search)
    };

    frame.render_widget(
        Paragraph::new(search_text)
            .block(
                Block::default()
                    .title("Filter")
                    .borders(Borders::ALL),
            ),
        layout[1],
    );

    //----------------------------------------------------------
    // Table
    //----------------------------------------------------------

    let header = Row::new(vec![
        "Process",
        "Live ↓",
        "Live ↑",
        "Session",
        "Total",
        "CPU",
        "RAM",
    ])
    .style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    );

    let table_rows = rows.iter().map(|row| {
        Row::new(vec![
            row.name.clone(),
            format!("{}/s", format_bytes(row.rx_speed)),
            format!("{}/s", format_bytes(row.tx_speed)),
            format_bytes(row.total_session()),
            format_bytes(row.total_process()),
            format!("{:.1}%", row.cpu),
            format_bytes(row.memory),
        ])
    });

    let table = Table::new(
        table_rows,
        [
            Constraint::Percentage(32),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(8),
            Constraint::Length(10),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title(" Dashboard ")
            .borders(Borders::ALL),
    )
    .highlight_symbol("▶ ")
    .row_highlight_style(
        Style::default()
            .bg(Color::Blue)
            .fg(Color::White),
    );

    let mut state = TableState::default();

    if !rows.is_empty() {
        state.select(Some(app.selected));
    }

    frame.render_stateful_widget(
        table,
        layout[2],
        &mut state,
    );

    //----------------------------------------------------------
    // Inspector
    //----------------------------------------------------------

    render_inspector(
        frame,
        layout[3],
        rows.get(app.selected),
    );
}

fn draw_card(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    value: String,
) {
    frame.render_widget(
        Paragraph::new(value)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title(title)
                    .borders(Borders::ALL),
            ),
        area,
    );
}
