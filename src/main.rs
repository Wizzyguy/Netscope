mod collector;
mod ui;

use collector::{
    collect_per_process_usage,
    discover_processes,
    discover_socket_inodes,
    filter_by_name,
    filter_idle,
    read_key,
    ThroughputTracker,
    KeyAction,
};

use ui::{App, SortMode};

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

    let mut tracker = ThroughputTracker::new();

    //--------------------------------------------------------
    // Main Loop
    //--------------------------------------------------------

    loop {
        //----------------------------------------------------
        // Discover Processes
        //----------------------------------------------------

        let processes = discover_processes();

        let active_pids: Vec<u32> =
            processes.iter()
                .map(|p| p.pid)
                .collect();

        tracker.cleanup(&active_pids);

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
        // Collect Usage
        //----------------------------------------------------

        let usage = collect_per_process_usage(sockets);

        let mut rows = Vec::new();

        for process in processes {

            if let Some((rx, tx)) =
                usage.get(&process.pid)
            {
                let (rx_speed, tx_speed) =
                    tracker.calculate(
                        process.pid,
                        *rx,
                        *tx,
                    );

                rows.push((
                    process.pid,
                    process.process_name,
                    *rx,
                    *tx,
                    rx_speed,
                    tx_speed,
                ));
            }
        }

        //----------------------------------------------------
        // Remove Idle Processes
        //----------------------------------------------------

        rows = filter_idle(rows);

        //----------------------------------------------------
        // Sorting
        //----------------------------------------------------

        match app.sort {

            SortMode::Download => {
                rows.sort_by(|a, b| b.2.cmp(&a.2));
            }

            SortMode::Upload => {
                rows.sort_by(|a, b| b.3.cmp(&a.3));
            }

            SortMode::Name => {
                rows.sort_by(|a, b|
                    a.1.to_lowercase()
                        .cmp(&b.1.to_lowercase())
                );
            }

            SortMode::Pid => {
                rows.sort_by(|a, b| a.0.cmp(&b.0));
            }
        }

        //----------------------------------------------------
        // Search
        //----------------------------------------------------

        if !app.search.is_empty() {
            rows = filter_by_name(rows, &app.search);
        }

        //----------------------------------------------------
        // Keep Selection Valid
        //----------------------------------------------------

        app.ensure_valid(rows.len());

        //----------------------------------------------------
        // Draw Dashboard
        //----------------------------------------------------

        terminal.draw(|frame| {
            ui::render_dashboard(
                frame,
                &app.search,
                app.search_mode,
                &rows,
                app.selected,
            );
        })?;

        //----------------------------------------------------
        // Keyboard Input
        //----------------------------------------------------

        match read_key() {

            KeyAction::Character(c) => {

                if app.search_mode {

                    app.search.push(c);

                } else {

                    match c {

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
