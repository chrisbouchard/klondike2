use std::convert;

use crate::klondike::{game, settings, table};
use crate::model;

#[derive(Debug, Clone, Default)]
pub struct KlondikeRules;

#[derive(Debug, Clone, Copy)]
pub struct KlondikeRulesContext<'a> {
    _settings: &'a settings::KlondikeSettings,
    _started: bool,
    _table: &'a table::KlondikeTable,
}

impl<'a> From<game::KlondikeGameRulesContext<'a>> for KlondikeRulesContext<'a> {
    fn from(context: game::KlondikeGameRulesContext<'a>) -> Self {
        Self {
            _settings: context.settings,
            _started: context.started,
            _table: context.table,
        }
    }
}

impl model::rules::Rules<table::KlondikeTableAction> for KlondikeRules {
    type Context<'a> = KlondikeRulesContext<'a>;
    type Error = convert::Infallible;

    fn validate(
        &self,
        _action: &table::KlondikeTableAction,
        _context: &Self::Context<'_>,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}
