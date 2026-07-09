pub struct App {
    pub selected: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            selected: 0,
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
