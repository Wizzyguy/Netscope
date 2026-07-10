use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

pub fn create_layout(frame: &mut Frame) {
    let _chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(3), // Stats
            Constraint::Length(3), // Search
            Constraint::Min(10),   // Table
            Constraint::Length(2), // Footer
        ])
        .split(frame.area());
}
