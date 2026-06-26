mod collector;

use std::collections::HashSet;

fn main() {
    println!("\n=== NetScope ===\n");

    let processes =
        collector::discover_processes();

    let tcp_rows =
        collector::read_tcp_table();

    let mut shown = 0;

    for process in processes {
        let sockets =
            collector::discover_socket_inodes(
                process.pid
            );

        if sockets.is_empty() {
            continue;
        }

        let socket_set =
            sockets
                .into_iter()
                .collect::<HashSet<_>>();

        let mut remotes =
            HashSet::<String>::new();

        for row in &tcp_rows {
            if row.len() < 10 {
                continue;
            }

            if socket_set.contains(
                &row[9]
            ) {
                remotes.insert(
                    row[2].clone()
                );
            }
        }

        if remotes.is_empty() {
            continue;
        }

        println!(
            "{} → {}",
            process.pid,
            process.process_name
        );

        let mut count = 0;

        for remote in remotes {
            println!(
                "→ {}",
                remote
            );

            count += 1;

            if count >= 5 {
                break;
            }
        }

        println!();

        shown += 1;

        if shown >= 10 {
            break;
        }
    }
}
