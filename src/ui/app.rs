pub enum SortMode {
    Download,
    Upload,
    Name,
    Pid,
}

pub struct App {
    pub selected: usize,
    pub search: String,
    pub search_mode: bool,
    pub sort: SortMode,
}

impl App {
    pub fn new() -> Self {
        Self {
            selected: 0,
            search: String::new(),
            search_mode: false,
            sort: SortMode::Download,
        }
    }

    pub fn next(&mut self, len: usize) {
        if len == 0 {
            return;
        }

        if self.selected < len - 1 {
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
}
