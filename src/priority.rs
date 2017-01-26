#[derive(Debug, Copy, Clone)]
pub enum Priority {
    Interactive,
    Default,
    InteractiveBulk,
    Bulk,
}

impl Default for Priority {
    fn default() -> Priority {
        Priority::Default
    }
}