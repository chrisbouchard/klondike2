#[derive(Debug, Clone, Copy)]
pub struct Settings {
    pub allow_move_from_foundation: bool,
    pub tableaux_width: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            allow_move_from_foundation: true,
            tableaux_width: 7,
        }
    }
}
