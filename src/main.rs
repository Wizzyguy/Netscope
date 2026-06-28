mod collector;

use collector::{
    collect_per_process_usage,
    discover_processes,
    discover_socket_inodes,
    filter_idle,
    sort_rows,
};

use std::{
    collections::HashMap,
    thread,
    time::Duration,
};

fn main() {

    println!("Starting NetScope...");

    loop {

        let processes =
            discover_processes();

        let mut sockets =
            HashMap::new();

        for process in &processes {

            sockets.insert(
                process.pid,
                discover_socket_inodes(
                    process.pid
                ),
            );
        }

        let usage =
            collect_per_process_usage(
                sockets
            );

        let mut rows =
            Vec::new();

        for process in processes {

            if let Some(
                (rx, tx)
            ) = usage.get(
                &process.pid
            ) {

                rows.push((
                    process.pid,
                    process.process_name,
                    *rx,
                    *tx,
                ));
            }
        }

        rows =
            filter_idle(rows);

        sort_rows(
            &mut rows
        );

        print!(
            "\x1B[2J\x1B[1;1H"
        );

        println!(
            "\n=== NetScope Dashboard ===\n"
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

        for (
            pid,
            name,
            rx,
            tx,
        ) in rows.iter().take(15)
        {

            println!(
                "{:<8} {:<18} {:<12} {:<12}",
                pid,
                name,
                rx,
                tx,
            );
        }

        println!(
            "\nFiltering idle processes..."
        );

        thread::sleep(
            Duration::from_secs(2)
        );
    }
}
