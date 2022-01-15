use crate::model;

use super::{dealer, settings};

pub type KlondikeGame<SH> = model::game::Game<dealer::KlondikeDealer, settings::Settings, SH>;
