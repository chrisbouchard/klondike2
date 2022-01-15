use crate::model;

use super::dealer;

pub type KlondikeGame<S> = model::game::Game<dealer::KlondikeDealer, S>;
