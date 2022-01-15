use crate::model;

use super::{dealer, rules, settings, table};

pub type KlondikeGame<SH> = model::game::Game<
    dealer::KlondikeDealer,
    rules::KlondikeRules,
    settings::Settings,
    SH,
    table::KlondikeTable,
>;
