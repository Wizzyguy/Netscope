use std::{
    collections::HashMap,
    fs,
};

pub fn build_inode_map() -> HashMap<u64, u32> {

    let mut map = HashMap::new();

    let proc = match fs::read_dir("/proc") {
        Ok(p) => p,
        Err(_) => return map,
    };

    for entry in proc.flatten() {

        let pid = match entry.file_name().to_string_lossy().parse::<u32>() {
            Ok(pid) => pid,
            Err(_) => continue,
        };

        let fd_dir = format!("/proc/{}/fd", pid);

        let fds = match fs::read_dir(fd_dir) {
            Ok(fds) => fds,
            Err(_) => continue,
        };

        for fd in fds.flatten() {

            let target = match fs::read_link(fd.path()) {
                Ok(link) => link,
                Err(_) => continue,
            };

            let target = target.to_string_lossy();

            if target.starts_with("socket:[") {

                let inode = target
                    .trim_start_matches("socket:[")
                    .trim_end_matches(']')
                    .parse::<u64>();

                if let Ok(inode) = inode {
                    map.insert(inode, pid);
                }
            }
        }
    }

    map
}
