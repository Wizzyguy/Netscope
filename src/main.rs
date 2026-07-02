mod collector;

use collector::{
    collect_per_process_usage,
    discover_processes,
    discover_socket_inodes,
    filter_by_name,
    filter_idle,
    sort_rows,
};

use std::{
    collections::HashMap,
    io,
    thread,
    time::Duration,
};

fn main() {
    println!("Starting NetScope...\n");

    println!("Enter process name to filter (leave blank for all):");

    let mut search = String::new();

    io::stdin()
        .read_line(&mut search)
        .expect("Failed to read input");

    let search = search.trim().to_string();

    loop {
        let processes = discover_processes();

        let mut sockets = HashMap::new();

        for process in &processes {
            sockets.insert(
                process.pid,
                discover_socket_inodes(process.pid),
            );
        }

        let usage =
            collect_per_process_usage(sockets);

        let mut rows = Vec::new();

        for process in processes {
            if let Some((rx, tx)) =
                usage.get(&process.pid)
            {
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
            rows =
                filter_by_name(rows, &search);
        }

        print!("\x1B[2J\x1B[1;1H");

        println!("\n=== NetScope Dashboard ===\n");

        if search.is_empty() {
            println!("Search Filter: ALL\n");
        } else {
            println!(
                "Search Filter: {}\n",
                search
            );
        }

        println!(
            "{:<8} {:<18} {:<12} {:<12}",
            "PID",
            "PROCESS",
            "RX",
            "TX"
        );

        println!(
            "------------------------------------------------"
        );

        for (pid, name, rx, tx)
            in rows.iter().take(15)
        {
            println!(
                "{:<8} {:<18} {:<12} {:<12}",
                pid,
                name,
                rx,
                tx,
            );
        }

        println!("\nRefreshing every 2 seconds...");

        thread::sleep(
            Duration::from_secs(2),
        );
    }
}
