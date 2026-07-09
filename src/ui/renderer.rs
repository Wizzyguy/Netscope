use crate::ui::format::format_bytes;

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
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(2),
        ])
        .split(frame.area());

    //----------------------------------------
    // Header
    //----------------------------------------

    let title = Paragraph::new("NetScope")
        .block(
            Block::default()
                .title("NetScope")
                .borders(Borders::ALL),
        );

    frame.render_widget(title, layout[0]);

    //----------------------------------------
    // Search Box
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

    frame.render_widget(search_widget, layout[1]);

    //----------------------------------------
    // Table
    //----------------------------------------

    let header = Row::new(vec![
        "PID",
        "Process",
        "RX",
        "TX",
    ])
    .style(Style::default().add_modifier(Modifier::BOLD));

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
            Constraint::Percentage(50),
            Constraint::Length(15),
            Constraint::Length(15),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title("Processes")
            .borders(Borders::ALL),
    );

    frame.render_widget(table, layout[2]);

    //----------------------------------------
    // Footer
    //----------------------------------------

    let footer = Paragraph::new(
        "/ Search    Esc Cancel    Enter Apply    q Quit",
    )
    .block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, layout[3]);
}
