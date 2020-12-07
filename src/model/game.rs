use super::selection;
use super::table;

pub enum GameAction {
    Draw,
    SelectLess,
    SelectMore,
    SendToFoundation,
    TakeFromWaste,
    ToggleSelection,
}

pub struct Game {
    table: table::Table,
    selection: selection::Selection,
}
