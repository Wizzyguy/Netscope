use ratatui::{prelude::*, widgets::*};

pub fn render_inspector(
    frame: &mut Frame,
    area: Rect,
    pid: u32,
    process: &str,
    memory: &str,
    cpu: &str,
    rx_total: &str,
    tx_total: &str,
) {
    let inspector = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("PID: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(pid.to_string()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Process: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(process),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("CPU: ", Style::default().fg(Color::Yellow)),
            Span::raw(cpu),
        ]),
        Line::from(vec![
            Span::styled("Memory: ", Style::default().fg(Color::Green)),
            Span::raw(memory),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("RX Total: ", Style::default().fg(Color::Cyan)),
            Span::raw(rx_total),
        ]),
        Line::from(vec![
            Span::styled("TX Total: ", Style::default().fg(Color::Magenta)),
            Span::raw(tx_total),
        ]),
    ])
    .block(Block::default().title("Inspector").borders(Borders::ALL))
    .wrap(Wrap { trim: true });

    frame.render_widget(inspector, area);
}
