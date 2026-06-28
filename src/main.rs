mod collector;

use std::{
    collections::HashMap,
    thread,
    time::Duration,
};

use collector::{
    collect_per_process_usage,
    discover_processes,
    discover_socket_inodes,
};

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    println!("Starting NetScope...");

    loop {
        clear_screen();

        println!("=== NetScope Dashboard ===\n");

        let processes = discover_processes();

        let mut process_sockets =
            HashMap::<u32, Vec<String>>::new();

        for process in &processes {
            let sockets =
                discover_socket_inodes(
                    process.pid,
                );

            process_sockets.insert(
                process.pid,
                sockets,
            );
        }

        let usage =
            collect_per_process_usage(
                process_sockets,
            );

        let mut rows:
            Vec<(u32, u64, u64)> =
            usage
                .into_iter()
                .map(
                    |(pid, (rx, tx))| {
                        (pid, rx, tx)
                    },
                )
                .collect();

        rows.sort_by(
            |a, b| {
                b.1.cmp(&a.1)
            },
        );

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

        for (pid, rx, tx)
            in rows.iter().take(15)
        {
            let name =
                processes
                    .iter()
                    .find(
                        |p| p.pid == *pid,
                    )
                    .map(
                        |p| {
                            p.process_name
                                .clone()
                        },
                    )
                    .unwrap_or(
                        String::from(
                            "unknown",
                        ),
                    );

            println!(
                "{:<8} {:<18} {:<12} {:<12}",
                pid,
                name,
                rx,
                tx
            );
        }

        println!(
            "\nRefreshing every 2 seconds..."
        );

        thread::sleep(
            Duration::from_secs(2),
        );
    }
}
