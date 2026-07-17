mod collector;
mod engine;
mod ui;

use collector::{
    collect_per_process_usage,
    discover_processes,
    discover_socket_inodes,
    filter_by_name,
    filter_idle,
    read_key,
    KeyAction,
};

use engine::Engine;

use ui::{
    render_ui,
    App,
    SortMode,
    Workspace,
};

use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

use std::{
    collections::HashMap,
    io::stdout,
    thread,
    time::Duration,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //--------------------------------------------------------
    // Terminal Setup
    //--------------------------------------------------------

    enable_raw_mode()?;

    let mut stdout = stdout();

    execute!(
        stdout,
        EnterAlternateScreen,
        Hide,
    )?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    //--------------------------------------------------------
    // Application State
    //--------------------------------------------------------

    let mut app = App::new();

    let mut engine = Engine::new();

    //--------------------------------------------------------
    // Main Loop
    //--------------------------------------------------------

    loop {
        //----------------------------------------------------
        // Discover Processes
        //----------------------------------------------------

        let processes = discover_processes();

        let active_pids: Vec<u32> =
            processes.iter().map(|p| p.pid).collect();

        engine.bandwidth.cleanup(&active_pids);
        engine.cpu.cleanup(&active_pids);
        engine.memory.cleanup(&active_pids);
        engine.session.cleanup(&active_pids);

        //----------------------------------------------------
        // Discover Sockets
        //----------------------------------------------------

        let mut sockets = HashMap::new();

        for process in &processes {
            sockets.insert(
                process.pid,
                discover_socket_inodes(process.pid),
            );
        }

        //----------------------------------------------------
        // Collect Network Usage
        //----------------------------------------------------

        let usage = collect_per_process_usage(sockets);

        //----------------------------------------------------
        // Engine Update
        //----------------------------------------------------

        engine.update(
            processes,
            usage,
        );

        //----------------------------------------------------
        // Dashboard Cache
        //----------------------------------------------------

        let mut rows = engine.dashboard().clone();

        //----------------------------------------------------
        // Remove Idle Processes
        //----------------------------------------------------

        rows = filter_idle(rows);

        //----------------------------------------------------
        // Sorting
        //----------------------------------------------------

        match app.sort {
            SortMode::Download => {
                rows.sort_by(|a, b| {
                    b.rx_speed.cmp(&a.rx_speed)
                });
            }

            SortMode::Upload => {
                rows.sort_by(|a, b| {
                    b.tx_speed.cmp(&a.tx_speed)
                });
            }

            SortMode::Name => {
                rows.sort_by(|a, b| {
                    a.name
                        .to_lowercase()
                        .cmp(&b.name.to_lowercase())
                });
            }

            SortMode::Pid => {
                rows.sort_by(|a, b| {
                    a.pid.cmp(&b.pid)
                });
            }
        }

        //----------------------------------------------------
        // Search
        //----------------------------------------------------

        if !app.search.is_empty() {
            rows = filter_by_name(
                rows,
                &app.search,
            );
        }

        //----------------------------------------------------
        // Keep Selection Valid
        //----------------------------------------------------

        app.ensure_valid(rows.len());

        //----------------------------------------------------
        // Draw UI
        //----------------------------------------------------

        terminal.draw(|frame| {
            render_ui(
                frame,
                &app,
                &rows,
                engine.connections(),
            );
        })?;

        //----------------------------------------------------
        // Keyboard
        //----------------------------------------------------

        match read_key() {
            KeyAction::Character(c) => {
                if app.search_mode {
                    app.search.push(c);
                } else {
                    match c {
                        'h' => {
                            app.workspace = Workspace::Dashboard;
                        }

                        'j' => {
                            app.workspace = Workspace::Connections;
                        }

                        'k' => {
                            app.workspace = Workspace::Security;
                        }

                        'l' => {
                            app.workspace = Workspace::Analytics;
                        }

                        '/' => {
                            app.search_mode = true;
                            app.search.clear();
                        }

                        'q' => break,

                        'd' => {
                            app.sort = SortMode::Download;
                        }

                        'u' => {
                            app.sort = SortMode::Upload;
                        }

                        'n' => {
                            app.sort = SortMode::Name;
                        }

                        'p' => {
                            app.sort = SortMode::Pid;
                        }

                        _ => {}
                    }
                }
            }

            KeyAction::Backspace => {
                if app.search_mode {
                    app.search.pop();
                }
            }

            KeyAction::Enter => {
                app.search_mode = false;
            }

            KeyAction::Esc => {
                app.search_mode = false;
                app.search.clear();
            }

            KeyAction::Up => {
                app.previous();
            }

            KeyAction::Down => {
                app.next(rows.len());
            }

            KeyAction::Left => {
                app.workspace = match app.workspace {
                    Workspace::Dashboard => Workspace::Dashboard,
                    Workspace::Connections => Workspace::Dashboard,
                    Workspace::Security => Workspace::Connections,
                    Workspace::Analytics => Workspace::Security,
                };
            }

            KeyAction::Right => {
                app.workspace = match app.workspace {
                    Workspace::Dashboard => Workspace::Connections,
                    Workspace::Connections => Workspace::Security,
                    Workspace::Security => Workspace::Analytics,
                    Workspace::Analytics => Workspace::Analytics,
                };
            }

            KeyAction::Reset => {
                engine.reset();
            }

            KeyAction::None => {}
        }

        //----------------------------------------------------
        // Refresh
        //----------------------------------------------------

        thread::sleep(Duration::from_millis(100));
    }

    //--------------------------------------------------------
    // Restore Terminal
    //--------------------------------------------------------

    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        Show,
        LeaveAlternateScreen,
    )?;

    terminal.show_cursor()?;

    println!("Exiting NetScope...");

    Ok(())
}
