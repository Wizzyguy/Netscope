mod collector;
mod ui;

use collector::{
    collect_per_process_usage,
    discover_processes,
    discover_socket_inodes,
    filter_by_name,
    filter_idle,
    read_key,
    sort_rows,
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
    //-------------------------------------------------------
    // Terminal Setup
    //-------------------------------------------------------

    enable_raw_mode()?;

    let mut stdout = stdout();

    execute!(
        stdout,
        EnterAlternateScreen,
        Hide,
    )?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    //-------------------------------------------------------
    // Application State
    //-------------------------------------------------------

    let mut app = App::new();

    //-------------------------------------------------------
    // Main Loop
    //-------------------------------------------------------

    loop {
        //---------------------------------------------------
        // Discover Processes
        //---------------------------------------------------

        let processes = discover_processes();

        let mut sockets = HashMap::new();

        for process in &processes {
            sockets.insert(
                process.pid,
                discover_socket_inodes(process.pid),
            );
        }

        //---------------------------------------------------
        // Collect Network Usage
        //---------------------------------------------------

        let usage = collect_per_process_usage(sockets);

        let mut rows = Vec::new();

        for process in processes {
            if let Some((rx, tx)) = usage.get(&process.pid) {
                rows.push((
                    process.pid,
                    process.process_name,
                    *rx,
                    *tx,
                ));
            }
        }

        //---------------------------------------------------
        // Remove idle processes
        //---------------------------------------------------

        rows = filter_idle(rows);

        //---------------------------------------------------
        // Temporary sorting
        //---------------------------------------------------

        sort_rows(&mut rows);

        //---------------------------------------------------
        // Search filtering
        //---------------------------------------------------

        if !app.search.is_empty() {
            rows = filter_by_name(rows, &app.search);
        }

        //---------------------------------------------------
        // Keep selected row valid
        //---------------------------------------------------

        app.ensure_valid(rows.len());

        //---------------------------------------------------
        // Draw UI
        //---------------------------------------------------

        terminal.draw(|frame| {
            ui::render_dashboard(
                frame,
                &app.search,
                app.search_mode,
                &rows,
                app.selected,
            );
        })?;

        //---------------------------------------------------
        // Keyboard
        //---------------------------------------------------

        match read_key() {
            KeyAction::Quit => break,

            //-----------------------------------------------
            // Search
            //-----------------------------------------------

            KeyAction::Search => {
                app.search_mode = true;
                app.search.clear();
            }

            KeyAction::Character(c) => {
                if app.search_mode {
                    app.search.push(c);
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

            //-----------------------------------------------
            // Navigation
            //-----------------------------------------------

            KeyAction::Up => {
                app.previous();
            }

            KeyAction::Down => {
                app.next(rows.len());
            }

            //-----------------------------------------------
            // Sort Modes (state only for now)
            //-----------------------------------------------

            KeyAction::SortDownload => {
                app.sort = SortMode::Download;
            }

            KeyAction::SortUpload => {
                app.sort = SortMode::Upload;
            }

            KeyAction::SortName => {
                app.sort = SortMode::Name;
            }

            KeyAction::SortPid => {
                app.sort = SortMode::Pid;
            }

            //-----------------------------------------------

            KeyAction::None => {}
        }

        thread::sleep(Duration::from_millis(100));
    }

    //-------------------------------------------------------
    // Restore Terminal
    //-------------------------------------------------------

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
