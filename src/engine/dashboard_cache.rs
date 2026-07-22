use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::collector::ProcessSession;

#[derive(Clone)]
pub struct CachedProcess {
    pub process: ProcessSession,
    pub last_seen: Instant,
    pub active: bool,
}

pub struct DashboardCache {
    rows: HashMap<u32, CachedProcess>,
    timeout: Duration,
}

impl DashboardCache {
    pub fn new() -> Self {
        Self {
            rows: HashMap::new(),
            timeout: Duration::from_secs(5),
        }
    }

    pub fn update(&mut self, processes: Vec<ProcessSession>) {
        let now = Instant::now();

        for process in processes {
            self.rows.insert(
                process.pid,
                CachedProcess {
                    process,
                    last_seen: now,
                    active: true,
                },
            );
        }

        for cached in self.rows.values_mut() {
            if now.duration_since(cached.last_seen) > self.timeout {
                cached.active = false;
            }
        }

        self.rows.retain(|_, process| {
            now.duration_since(process.last_seen)
                <= self.timeout
        });
    }

    pub fn rows(&self) -> Vec<ProcessSession> {
        let mut rows = self
            .rows
            .values()
            .map(|p| p.process.clone())
            .collect::<Vec<_>>();

        rows.sort_by(|a, b| b.rx_speed.cmp(&a.rx_speed));

        rows
    }

    pub fn clear(&mut self) {
        self.rows.clear();
    }
}
