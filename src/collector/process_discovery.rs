use std::fs;

use super::models::ProcessInfo;

fn read_memory_kb(pid: u32) -> u64 {
    let status_path = format!("/proc/{}/status", pid);

    if let Ok(content) = fs::read_to_string(status_path) {
        for line in content.lines() {
            if line.starts_with("VmRSS:") {
                let parts: Vec<&str> =
                    line.split_whitespace().collect();

                if let Some(value) = parts.get(1) {
                    return value.parse().unwrap_or(0);
                }
            }
        }
    }

    0
}

fn read_executable(pid: u32) -> String {
    let exe_path =
        format!("/proc/{}/exe", pid);

    fs::read_link(exe_path)
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "unknown".to_string())
}

pub fn discover_processes() -> Vec<ProcessInfo> {
    let mut processes = Vec::new();

    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries.flatten() {
            let pid_name = entry.file_name();
            let pid_str =
                pid_name.to_string_lossy();

            if let Ok(pid) = pid_str.parse::<u32>() {
                let comm_path =
                    format!("/proc/{}/comm", pid);

                let process_name =
                    fs::read_to_string(comm_path)
                        .unwrap_or_else(|_| {
                            "unknown".to_string()
                        })
                        .trim()
                        .to_string();

                let memory_kb =
                    read_memory_kb(pid);

                let executable_path =
                    read_executable(pid);

                if memory_kb > 0 {
                    processes.push(ProcessInfo {
                        pid,
                        process_name,
                        memory_kb,
                        executable_path,
                    });
                }
            }
        }
    }

    processes
}
