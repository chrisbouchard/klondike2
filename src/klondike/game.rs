use crate::model;

use super::{dealer, rules, settings};

pub type KlondikeGame<SH> =
    model::game::Game<dealer::KlondikeDealer, rules::KlondikeRules, settings::Settings, SH>;
