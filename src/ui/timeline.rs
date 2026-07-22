use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::engine::ProcessEvent;

pub fn draw_timeline(
    frame: &mut Frame,
    area: Rect,
    events: &[ProcessEvent],
) {
    let items: Vec<ListItem> = events
        .iter()
        .rev()
        .take(50)
        .map(|event| {
            ListItem::new(format!(
                "[{:?}] {} ({}) - {}",
                event.timestamp,
                event.process,
                event.pid,
                event.message
            ))
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title("Timeline")
            .borders(Borders::ALL),
    );

    frame.render_widget(list, area);
}
