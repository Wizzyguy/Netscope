mod collector;

use collector::network_usage::collect_network_usage;

fn main() {
    println!("Starting NetScope...\n");

    collect_network_usage();
}
