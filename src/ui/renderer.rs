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

    // ---------------- Header ----------------

    let header = Paragraph::new("NetScope")
        .block(
            Block::default()
                .title("Network Monitor")
                .borders(Borders::ALL),
        )
        .alignment(Alignment::Center);

    frame.render_widget(header, layout[0]);

    // ---------------- Search ----------------

    let search_text = if search_mode {
        format!("Search: {}_", search)
    } else if search.is_empty() {
        "Search: None".to_string()
    } else {
        format!("Search: {}", search)
    };

    let search_box = Paragraph::new(search_text)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(search_box, layout[1]);

    // ---------------- Table ----------------

    let table_rows: Vec<Row> = rows
        .iter()
        .take(20)
        .map(|(pid, name, rx, tx)| {
            Row::new(vec![
                pid.to_string(),
                name.clone(),
                rx.to_string(),
                tx.to_string(),
            ])
        })
        .collect();

    let table = Table::new(
        table_rows,
        [
            Constraint::Length(8),
            Constraint::Length(30),
            Constraint::Length(15),
            Constraint::Length(15),
        ],
    )
    .header(
        Row::new(vec!["PID", "Process", "RX", "TX"])
            .style(Style::default().add_modifier(Modifier::BOLD)),
    )
    .block(Block::default().title("Processes").borders(Borders::ALL));

    frame.render_widget(table, layout[2]);

    // ---------------- Footer ----------------

    let footer = Paragraph::new("/ Search | Esc Cancel | q Quit")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, layout[3]);
}
