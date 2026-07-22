use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::collector::ProcessSession;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    New,
    Active,
    Idle,
    Archived,
}

#[derive(Clone)]
pub struct LifecycleRecord {
    pub process: ProcessSession,

    pub state: ProcessState,

    pub first_seen: Instant,

    pub last_seen: Instant,

    pub idle_since: Option<Instant>,

    pub archived_at: Option<Instant>,
}

pub struct LifecycleEngine {
    processes: HashMap<u32, LifecycleRecord>,

    idle_timeout: Duration,

    archive_timeout: Duration,
}
