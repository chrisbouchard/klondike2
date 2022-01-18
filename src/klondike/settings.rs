#[derive(Debug, Clone, Copy)]
pub struct KlondikeSettings {
    pub allow_move_from_foundation: bool,
    pub tableaux_width: usize,
}

impl Default for KlondikeSettings {
    fn default() -> Self {
        Self {
            allow_move_from_foundation: true,
            tableaux_width: 7,
        }
    }
}
