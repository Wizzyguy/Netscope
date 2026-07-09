use ratatui::{
    prelude::*,
    widgets::*,
};

pub fn render_inspector(
    frame: &mut Frame,
    area: Rect,
    pid: u32,
    process: &str,
    rx: &str,
    tx: &str,
) {
    let text = vec![
        Line::from(format!("PID        : {}", pid)),
        Line::from(format!("Process    : {}", process)),
        Line::from(""),
        Line::from(format!("Download   : {}", rx)),
        Line::from(format!("Upload     : {}", tx)),
        Line::from(""),
        Line::from("Memory      : Coming Soon"),
        Line::from("Executable  : Coming Soon"),
        Line::from("Sockets     : Coming Soon"),
    ];

    let widget = Paragraph::new(text)
        .block(
            Block::default()
                .title("Inspector")
                .borders(Borders::ALL),
        );

    frame.render_widget(widget, area);
}
