use ratatui::{
    prelude::*,
    widgets::*,
};

pub fn render_security_view(
    frame: &mut Frame,
    area: Rect,
) {

    let text = Paragraph::new(
        "Security Workspace\n\n\
Coming Soon\n\n\
• Digital Signatures\n\
• SHA256 Hashes\n\
• Reputation\n\
• Risk Analysis\n\
• Country Detection\n\
• VirusTotal Integration"
    )
    .block(
        Block::default()
            .title("Security")
            .borders(Borders::ALL),
    );

    frame.render_widget(
        text,
        area,
    );

}
