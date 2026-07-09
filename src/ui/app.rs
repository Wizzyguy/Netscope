#[derive(Default)]
pub struct App {
    pub search: String,
    pub search_mode: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            search: String::new(),
            search_mode: false,
        }
    }
}
