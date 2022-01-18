use crate::klondike::{dealer, rules, settings, table};
use crate::model;

pub type KlondikeGame<SH> = model::game::Game<
    dealer::KlondikeDealer,
    rules::KlondikeRules,
    settings::KlondikeSettings,
    SH,
    table::KlondikeTable,
>;

pub type KlondikeGameDealerContext<'a> =
    model::game::GameDealerContext<'a, settings::KlondikeSettings>;
pub type KlondikeGameRulesContext<'a> =
    model::game::GameRulesContext<'a, settings::KlondikeSettings, table::KlondikeTable>;
