mod collector;

use std::thread;
use std::time::Duration;

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    loop {
        clear_terminal();

        let processes =
            collector::collect();

        println!(
            "=== NetScope ===\n"
        );

        for process in processes.iter().take(5) {
            let sockets =
                collector::discover_socket_inodes(
                    process.pid,
                );

            println!(
                "{} → {}",
                process.pid,
                process.process_name
            );

            println!(
                "Unique sockets: {}",
                sockets.len()
            );

            for inode in sockets.iter().take(5) {
                println!(
                    "socket:[{}]",
                    inode
                );
            }

            println!();
        }

        thread::sleep(
            Duration::from_secs(1)
        );
    }
}
