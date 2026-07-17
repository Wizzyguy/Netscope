use ratatui::{
    prelude::*,
    widgets::*,
};

pub fn render_analytics_view(
    frame: &mut Frame,
    area: Rect,
) {

    let text = Paragraph::new(
        "Analytics Workspace\n\n\
Coming Soon\n\n\
• Behavioral Analysis\n\
• Session Statistics\n\
• Bandwidth Forecasting\n\
• Network Health\n\
• Recommendations"
    )
    .block(
        Block::default()
            .title("Analytics")
            .borders(Borders::ALL),
    );

    frame.render_widget(
        text,
        area,
    );

}
