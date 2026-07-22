#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Workspace {
    Dashboard,
    Connections,
    Security,
    Analytics,
    Timeline,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SortMode {
    Download,
    Upload,
    Name,
    Pid,
}

pub struct App {
    //--------------------------------------------------
    // Workspace
    //--------------------------------------------------

    pub workspace: Workspace,

    //--------------------------------------------------
    // Selected Row
    //--------------------------------------------------

    pub selected: usize,
    pub selected_pid: Option<u32>,

    //--------------------------------------------------
    // Search
    //--------------------------------------------------

    pub search: String,
    pub search_mode: bool,

    //--------------------------------------------------
    // Sorting
    //--------------------------------------------------

    pub sort: SortMode,

    //--------------------------------------------------
    // Session Totals
    //--------------------------------------------------

    pub total_download: u64,
    pub total_upload: u64,

    //--------------------------------------------------
    // Controls
    //--------------------------------------------------

    pub reset_requested: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            workspace: Workspace::Dashboard,

            selected: 0,
            selected_pid: None,

            search: String::new(),
            search_mode: false,

            sort: SortMode::Download,

            total_download: 0,
            total_upload: 0,

            reset_requested: false,
        }
    }

    //--------------------------------------------------
    // Workspace Navigation
    //--------------------------------------------------

    pub fn next_workspace(&mut self) {
        self.workspace = match self.workspace {
            Workspace::Dashboard => Workspace::Connections,
            Workspace::Connections => Workspace::Security,
            Workspace::Security => Workspace::Analytics,
            Workspace::Analytics => Workspace::Timeline,
            Workspace::Timeline => Workspace::Dashboard,
        };
    }

    pub fn previous_workspace(&mut self) {
        self.workspace = match self.workspace {
            Workspace::Dashboard => Workspace::Timeline,
            Workspace::Connections => Workspace::Dashboard,
            Workspace::Security => Workspace::Connections,
            Workspace::Analytics => Workspace::Security,
            Workspace::Timeline => Workspace::Analytics,        };
    }

    pub fn workspace_title(&self) -> &'static str {
        match self.workspace {
            Workspace::Dashboard => "Dashboard",
            Workspace::Connections => "Connections",
            Workspace::Security => "Security",
            Workspace::Analytics => "Analytics",
            Workspace::Timeline => "Timeline",
        }
    }

    //--------------------------------------------------
    // Row Navigation
    //--------------------------------------------------

    pub fn next(&mut self, len: usize) {
        if len == 0 {
            self.selected = 0;
            return;
        }

        if self.selected + 1 < len {
            self.selected += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn ensure_valid(&mut self, len: usize) {
        if len == 0 {
            self.selected = 0;
        } else if self.selected >= len {
            self.selected = len - 1;
        }
    }

    //--------------------------------------------------
    // Selection Persistence
    //--------------------------------------------------

    pub fn remember_selection(
        &mut self,
        pid: Option<u32>,
    ) {
        self.selected_pid = pid;
    }

    pub fn restore_selection(
        &mut self,
        rows: &[crate::collector::ProcessSession],
    ) {
        if let Some(pid) = self.selected_pid {
            if let Some(index) = rows
                .iter()
                .position(|row| row.pid == pid)
            {
                self.selected = index;
            }
        }
    }

    //--------------------------------------------------
    // Search
    //--------------------------------------------------

    pub fn clear_search(&mut self) {
        self.search.clear();
        self.search_mode = false;
    }

    //--------------------------------------------------
    // Session Totals
    //--------------------------------------------------

    pub fn update_totals(
        &mut self,
        download: u64,
        upload: u64,
    ) {
        self.total_download = download;
        self.total_upload = upload;
    }

    pub fn total_bandwidth(&self) -> u64 {
        self.total_download + self.total_upload
    }

    //--------------------------------------------------
    // Reset
    //--------------------------------------------------

    pub fn request_reset(&mut self) {
        self.reset_requested = true;
    }

    pub fn clear_reset(&mut self) {
        self.reset_requested = false;
    }
}
