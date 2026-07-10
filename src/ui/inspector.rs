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
    rx_speed: &str,
    tx_speed: &str,
) {
    let text = vec![
        Line::from(format!("PID       : {}", pid)),
        Line::from(format!("Process   : {}", process)),
        Line::from(""),
        Line::from(format!("RX Total  : {}", rx)),
        Line::from(format!("TX Total  : {}", tx)),
        Line::from(""),
        Line::from(format!("RX Speed  : {}", rx_speed)),
        Line::from(format!("TX Speed  : {}", tx_speed)),
    ];

    let panel = Paragraph::new(text)
        .block(
            Block::default()
                .title("Inspector")
                .borders(Borders::ALL),
        );

    frame.render_widget(panel, area);
}
