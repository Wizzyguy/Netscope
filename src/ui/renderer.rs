use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::{
    collector::{
        ConnectionInfo,
        ProcessSession,
    },
    ui::{
        render_analytics_view,
        render_connections_view,
        render_dashboard_view,
        render_footer,
        render_header,
        render_security_view,
        App,
        Workspace,
    },
};

pub fn render_ui(
    frame: &mut Frame,
    app: &App,
    dashboard_rows: &Vec<ProcessSession>,
    connections: &Vec<ConnectionInfo>,
) {
    //--------------------------------------------------------
    // Entire Screen
    //--------------------------------------------------------

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(10),   // Workspace
            Constraint::Length(2), // Footer
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
