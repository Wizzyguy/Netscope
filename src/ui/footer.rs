use ratatui::{
    prelude::*,
    widgets::*,
};

use crate::ui::{App, Workspace};

pub fn render_footer(
    frame: &mut Frame,
    area: Rect,
    app: &App,
) {
    let text = match app.workspace {
        Workspace::Dashboard => {
            "←→ Workspace   ↑↓ Select   / Search   R Reset Totals   Q Quit"
        }

        Workspace::Connections => {
            "←→ Workspace   ↑↓ Select Connection   Q Quit"
        }

        Workspace::Security => {
            "←→ Workspace   ↑↓ Select Process   Q Quit"
        }

        Workspace::Analytics => {
            "←→ Workspace   Q Quit"
        }
    };

    let footer = Paragraph::new(text)
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(Color::DarkGray),
        )
        .block(
            Block::default()
                .borders(Borders::TOP),
        );

    frame.render_widget(footer, area);
}
