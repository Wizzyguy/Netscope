#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Workspace {
    Dashboard,
    Connections,
    Security,
    Analytics,
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
            Workspace::Analytics => Workspace::Dashboard,
        };
    }

    pub fn previous_workspace(&mut self) {
        self.workspace = match self.workspace {
            Workspace::Dashboard => Workspace::Analytics,
            Workspace::Connections => Workspace::Dashboard,
            Workspace::Security => Workspace::Connections,
            Workspace::Analytics => Workspace::Security,
        };
    }

    pub fn workspace_title(&self) -> &'static str {
        match self.workspace {
            Workspace::Dashboard => "Dashboard",
            Workspace::Connections => "Connections",
            Workspace::Security => "Security",
            Workspace::Analytics => "Analytics",
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
