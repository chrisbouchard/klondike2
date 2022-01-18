use std::convert;

use crate::model;

use super::{game, settings, table};

#[derive(Debug, Clone, Default)]
pub struct KlondikeRules;

#[derive(Debug, Clone, Copy)]
pub struct KlondikeRulesContext<'a> {
    settings: &'a settings::KlondikeSettings,
    started: bool,
    table: &'a table::KlondikeTable,
}

impl<'a> From<game::KlondikeGameRulesContext<'a>> for KlondikeRulesContext<'a> {
    fn from(context: game::KlondikeGameRulesContext<'a>) -> Self {
        Self {
            settings: context.settings,
            started: context.started,
            table: context.table,
        }
    }
}

impl model::rules::Rules<table::KlondikeTableAction> for KlondikeRules {
    type Context<'a> = KlondikeRulesContext<'a>;
    type Error = convert::Infallible;

    fn validate(
        &self,
        action: &table::KlondikeTableAction,
        context: &Self::Context<'_>,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}
