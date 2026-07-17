use ratatui::{
    prelude::*,
    widgets::*,
};

use crate::collector::ConnectionInfo;

pub fn render_connections_view(
    frame: &mut Frame,
    area: Rect,
    connections: &Vec<ConnectionInfo>,
) {

    //--------------------------------------------------------
    // Table Header
    //--------------------------------------------------------

    let header = Row::new(vec![
        "Process",
        "Protocol",
        "Remote Host",
        "Port",
        "State",
    ])
    .style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    );

    //--------------------------------------------------------
    // Table Rows
    //--------------------------------------------------------

    let rows = connections.iter().map(|connection| {

        Row::new(vec![

            connection.process_name.clone(),

            connection.protocol.clone(),

            connection.remote_host.clone(),

            connection.port.to_string(),

            connection.state.clone(),

        ])

    });

    //--------------------------------------------------------
    // Table
    //--------------------------------------------------------

    let table = Table::new(
        rows,
        [

            Constraint::Percentage(28),

            Constraint::Length(10),

            Constraint::Percentage(36),

            Constraint::Length(8),

            Constraint::Length(14),

        ],
    )
    .header(header)
    .block(
        Block::default()
            .title(" Live Connections ")
            .borders(Borders::ALL),
    )
    .row_highlight_style(
        Style::default()
            .bg(Color::Blue)
            .fg(Color::White),
    )
    .highlight_symbol("▶ ");

    let mut state = TableState::default();

    if !connections.is_empty() {
        state.select(Some(0));
    }

    frame.render_stateful_widget(
        table,
        area,
        &mut state,
    );

}
