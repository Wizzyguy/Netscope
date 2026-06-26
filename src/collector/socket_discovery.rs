use std::collections::HashSet;
use std::fs;

pub fn discover_socket_inodes(
    pid: u32,
) -> Vec<String> {
    let fd_path =
        format!(
            "/proc/{}/fd",
            pid
        );

    let entries =
        match fs::read_dir(
            fd_path
        ) {
            Ok(v) => v,

            Err(_) => {
                return vec![];
            }
        };

    let mut unique =
        HashSet::new();

    for entry in entries {
        if let Ok(entry) =
            entry
        {
            if let Ok(target) =
                fs::read_link(
                    entry.path()
                )
            {
                let target =
                    target
                        .to_string_lossy();

                if target.starts_with(
                    "socket:["
                ) {
                    let inode =
                        target
                            .replace(
                                "socket:[",
                                "",
                            )
                            .replace(
                                "]",
                                "",
                            );

                    unique.insert(
                        inode
                    );
                }
            }
        }
    }

    let mut sockets =
        unique
            .into_iter()
            .collect::<Vec<_>>();

    sockets.sort();

    sockets
}
