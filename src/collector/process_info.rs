use std::fs;

pub fn executable(pid: u32) -> String {

    fs::read_link(format!("/proc/{}/exe", pid))
        .ok()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "-".to_string())

}

pub fn username(pid: u32) -> String {

    let status = fs::read_to_string(
        format!("/proc/{}/status", pid)
    ).unwrap_or_default();

    for line in status.lines() {

        if line.starts_with("Uid:") {

            return line
                .split_whitespace()
                .nth(1)
                .unwrap_or("-")
                .to_string();

        }

    }

    "-".to_string()

}

pub fn process_name(pid: u32) -> String {

    fs::read_to_string(
        format!("/proc/{}/comm", pid)
    )
    .unwrap_or_default()
    .trim()
    .to_string()

}
