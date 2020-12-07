//! The core model of the Klondike game. The types in this module represent the state of the game,
//! but not the display of the game.

pub mod card;
pub mod game;
pub mod pile;
pub mod selection;
pub mod table;

// TODO: Should I expand these globs?
pub use card::*;
pub use pile::*;
pub use selection::*;
pub use table::*;
