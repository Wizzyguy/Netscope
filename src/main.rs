mod collector;

fn main() {
    println!("Starting NetScope...");

    let processes =
        collector::collect();

    println!(
        "Processes discovered: {}",
        processes.len()
    );

    for process in processes.iter().take(10) {
        println!(
            "{} → {} ({} KB)",
            process.pid,
            process.process_name,
            process.memory_kb
        );

        println!(
            "   {}",
            process.executable_path
        );
    }
}
