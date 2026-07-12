use std::fs;

pub fn read_process_cpu(pid: u32) -> Option<u64> {
    let path = format!("/proc/{}/stat", pid);

    let content = fs::read_to_string(path).ok()?;

    let fields: Vec<&str> = content.split_whitespace().collect();

    if fields.len() < 17 {
        return None;
    }

    let utime = fields[13].parse::<u64>().ok()?;
    let stime = fields[14].parse::<u64>().ok()?;

    Some(utime + stime)
}
