use std::collections::HashMap;

pub fn collect_per_process_usage(
    process_sockets: HashMap<u32, Vec<String>>,
) -> HashMap<u32, (u64, u64)> {
    let mut usage = HashMap::new();

    for (pid, sockets) in process_sockets {
        let active = sockets.len() as u64;

        if active == 0 {
            continue;
        }

        // temporary weighting
        let rx = active * active * 1024;
        let tx = active * active * 512;

        usage.insert(pid, (rx, tx));
    }

    usage
}
