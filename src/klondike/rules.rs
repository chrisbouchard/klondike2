use crate::model;

use super::{game, settings};

#[derive(Debug, Clone, Default)]
pub struct KlondikeRules;

#[derive(Debug, Clone, Copy)]
pub struct KlondikeRulesContext<'a> {
    settings: &'a settings::Settings,
    started: bool,
    table: &'a model::table::Table,
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

impl model::rules::Rules<model::table::Action> for KlondikeRules {
    type Context<'a> = KlondikeRulesContext<'a>;
    type Error = ();

    fn is_valid(
        &self,
        action: &model::table::Action,
        context: Self::Context<'_>,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}
