use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::{
    collector::{
        ConnectionInfo,
        ProcessSession,
    },
    engine::ProcessEvent,
    ui::{
        render_analytics_view,
        render_connections_view,
        render_dashboard_view,
        render_footer,
        render_header,
        render_security_view,
        draw_timeline,
        App,
        Workspace,
    },
};

pub fn render_ui(
    frame: &mut Frame,
    app: &App,
    dashboard_rows: &Vec<ProcessSession>,
    connections: &Vec<ConnectionInfo>,
    events: &[ProcessEvent],
) {
    //--------------------------------------------------------
    // Entire Screen
    //--------------------------------------------------------

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(2),
        ])
        .split(frame.area());

    //--------------------------------------------------------
    // Header
    //--------------------------------------------------------

    render_header(
        frame,
        layout[0],
        app,
    );

    //--------------------------------------------------------
    // Workspace Router
    //--------------------------------------------------------

    match app.workspace {
        Workspace::Dashboard => {
            render_dashboard_view(
                frame,
                layout[1],
                app,
                dashboard_rows,
            );
        }

        Workspace::Connections => {
            render_connections_view(
                frame,
                layout[1],
                connections,
            );
        }

        Workspace::Security => {
            render_security_view(
                frame,
                layout[1],
            );
        }

        Workspace::Analytics => {
            render_analytics_view(
                frame,
                layout[1],
            );
        }

        Workspace::Timeline => {
            draw_timeline(
                frame,
                layout[1],
                events,
            );
        }
    }

    //--------------------------------------------------------
    // Footer
    //--------------------------------------------------------

    render_footer(
        frame,
        layout[2],
        app,
    );
}
