use std::fs;

use super::models::ProcessInfo;

pub fn discover_processes() -> Vec<ProcessInfo> {
    let mut processes = Vec::new();

    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let pid_str = name.to_string_lossy();

            if let Ok(pid) = pid_str.parse::<u32>() {
                let comm_path = format!("/proc/{}/comm", pid);

                let process_name = fs::read_to_string(comm_path)
                    .unwrap_or_else(|_| "unknown".to_string())
                    .trim()
                    .to_string();

                // Skip obvious kernel threads
                if !process_name.starts_with('[')
                    && process_name != "kthreadd"
                    && !process_name.starts_with("kworker")
                {
                    processes.push(ProcessInfo {
                        pid,
                        process_name,
                    });
                }
            }
        }
    }

    processes
}
