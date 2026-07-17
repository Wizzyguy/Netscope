use ratatui::{
    prelude::*,
    widgets::*,
};

use crate::ui::{App, Workspace};

pub fn render_header(
    frame: &mut Frame,
    area: Rect,
    app: &App,
) {
    let mut tabs = Vec::new();

    tabs.push(tab("Dashboard", app.workspace == Workspace::Dashboard));
    tabs.push(tab("Connections", app.workspace == Workspace::Connections));
    tabs.push(tab("Security", app.workspace == Workspace::Security));
    tabs.push(tab("Analytics", app.workspace == Workspace::Analytics));

    let header = Paragraph::new(Line::from(tabs))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title("NetScope 0.3")
                .borders(Borders::ALL),
        );

    frame.render_widget(header, area);
}

fn tab(
    title: &str,
    active: bool,
) -> Span<'static> {
    if active {
        Span::styled(
            format!(" [{}] ", title),
            Style::default()
                .fg(Color::Black)
                .bg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        Span::styled(
            format!(" {} ", title),
            Style::default().fg(Color::Gray),
        )
    }
}
