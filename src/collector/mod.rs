mod models;
mod process_discovery;

pub fn initialize() {
    println!("Collector initialized");

    let processes = process_discovery::discover_processes();

    println!("Processes discovered: {}", processes.len());

    for process in processes.iter().take(10) {
        println!(
            "{} → {}",
            process.pid,
            process.process_name
        );
    }
}
