use crate::model;

use super::{game, settings, table};

#[derive(Debug, Clone, Default)]
pub struct KlondikeRules;

#[derive(Debug, Clone, Copy)]
pub struct KlondikeRulesContext<'a> {
    settings: &'a settings::Settings,
    started: bool,
    table: &'a table::KlondikeTable,
}

impl<'a, SH> From<&'a game::KlondikeGame<SH>> for KlondikeRulesContext<'a> {
    fn from(game: &'a game::KlondikeGame<SH>) -> Self {
        Self {
            settings: game.settings(),
            started: game.is_started(),
            table: game.table(),
        }
    }
}

impl model::rules::Rules<table::KlondikeTableAction> for KlondikeRules {
    type Context<'a> = KlondikeRulesContext<'a>;
    type Error = ();

    fn is_valid(
        &self,
        action: &table::KlondikeTableAction,
        context: Self::Context<'_>,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}
