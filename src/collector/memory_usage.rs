use std::fs;

pub fn read_process_memory(pid: u32) -> Option<u64> {
    let path = format!("/proc/{}/status", pid);

    let content = fs::read_to_string(path).ok()?;

    for line in content.lines() {
        if line.starts_with("VmRSS:") {
            let value = line.split_whitespace().nth(1)?.parse::<u64>().ok()?;

            return Some(value);
        }
    }

    None
}
