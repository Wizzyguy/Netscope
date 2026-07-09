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

use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

use crossterm::{
    execute,
    terminal::{
        enable_raw_mode,
        disable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

use std::{
    collections::HashMap,
    io::stdout,
    time::{Duration, Instant},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Terminal setup
    enable_raw_mode()?;

    let mut stdout = stdout();

    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    let mut search = String::new();
    let mut search_mode = false;

    let mut last_refresh = Instant::now();

    loop {
        // ----------------------------
        // Keyboard
        // ----------------------------

        match read_key() {
            KeyAction::Quit => break,

            KeyAction::Search => {
                search_mode = true;
                search.clear();
            }

            KeyAction::Character(c) => {
                if search_mode {
                    search.push(c);
                }
            }

            KeyAction::Backspace => {
                if search_mode {
                    search.pop();
                }
            }

            KeyAction::Enter => {
                search_mode = false;
            }

            KeyAction::Esc => {
                search_mode = false;
                search.clear();
            }

            KeyAction::None => {}
        }

        // ----------------------------
        // Refresh every 300 ms
        // ----------------------------

        if last_refresh.elapsed() >= Duration::from_millis(300) {

            let processes = discover_processes();

            let mut sockets = HashMap::new();

            for process in &processes {
                sockets.insert(
                    process.pid,
                    discover_socket_inodes(process.pid),
                );
            }

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

            rows = filter_idle(rows);

            sort_rows(&mut rows);

            if !search.is_empty() {
                rows = filter_by_name(rows, &search);
            }

            terminal.draw(|frame| {
                ui::render_dashboard(
                    frame,
                    &search,
                    search_mode,
                    &rows,
                );
            })?;

            last_refresh = Instant::now();
        }
    }

    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;

    terminal.show_cursor()?;

    println!("Exiting NetScope...");

    Ok(())
}
