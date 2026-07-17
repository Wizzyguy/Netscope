use ratatui::{
    prelude::*,
    widgets::*,
};

use crate::{
    collector::ProcessSession,
    ui::format::format_bytes,
};

pub fn render_inspector(
    frame: &mut Frame,
    area: Rect,
    process: Option<&ProcessSession>,
) {
    let block = Block::default()
        .title(" Inspector ")
        .borders(Borders::ALL);

    match process {
        None => {
            frame.render_widget(
                Paragraph::new("No process selected")
                    .block(block)
                    .alignment(Alignment::Center),
                area,
            );
        }

        Some(p) => {
            let status = if p.rx_speed > 0 || p.tx_speed > 0 {
                Span::styled(
                    "ACTIVE",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled(
                    "IDLE",
                    Style::default()
                        .fg(Color::DarkGray),
                )
            };

            let text = vec![
                Line::from(vec![
                    Span::styled(
                        "Process : ",
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(&p.name),
                ]),

                Line::from(vec![
                    Span::styled(
                        "PID     : ",
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(p.pid.to_string()),
                ]),

                Line::from(""),

                Line::from(vec![
                    Span::styled(
                        "CPU     : ",
                        Style::default().fg(Color::Yellow),
                    ),
                    Span::raw(format!("{:.1} %", p.cpu)),
                ]),

                Line::from(vec![
                    Span::styled(
                        "Memory  : ",
                        Style::default().fg(Color::Green),
                    ),
                    Span::raw(format_bytes(p.memory)),
                ]),

                Line::from(""),

                Line::from(vec![
                    Span::styled(
                        "RX/s    : ",
                        Style::default().fg(Color::Cyan),
                    ),
                    Span::raw(format!("{}/s", format_bytes(p.rx_speed))),
                ]),

                Line::from(vec![
                    Span::styled(
                        "TX/s    : ",
                        Style::default().fg(Color::Magenta),
                    ),
                    Span::raw(format!("{}/s", format_bytes(p.tx_speed))),
                ]),

                Line::from(""),

                Line::from(vec![
                    Span::styled(
                        "Process Total : ",
                        Style::default().fg(Color::Blue),
                    ),
                    Span::raw(format_bytes(p.total_process())),
                ]),

                Line::from(vec![
                    Span::styled(
                        "Session Total : ",
                        Style::default().fg(Color::Blue),
                    ),
                    Span::raw(format_bytes(p.total_session())),
                ]),

                Line::from(""),

                Line::from(vec![
                    Span::styled(
                        "Status : ",
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    status,
                ]),
            ];

            frame.render_widget(
                Paragraph::new(text)
                    .block(block)
                    .wrap(Wrap { trim: true }),
                area,
            );
        }
    }
}
