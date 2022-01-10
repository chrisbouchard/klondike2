use super::action::Actionable as _;

use super::action;
use super::rules;
use super::table;

#[derive(Debug, Clone)]
pub struct Game<R> {
    rules: R,
    started: bool,
    table: table::Table,
}

#[derive(Clone, Copy, Debug)]
pub struct GameState<'a> {
    pub started: bool,
    pub table: &'a table::Table,
}

impl<R> Game<R> {
    pub fn rules(&self) -> &R {
        &self.rules
    }

    pub fn state(&self) -> GameState<'_> {
        GameState {
            started: self.is_started(),
            table: self.table(),
        }
    }

    pub fn table(&self) -> &table::Table {
        &self.table
    }

    pub fn is_started(&self) -> bool {
        self.started
    }
}

impl<R> Game<R>
where
    R: for<'a> rules::Rules<table::Action, table::Table, State<'a> = GameState<'a>>,
{
    // TODO: Possibly replace with a builder
    pub fn new(rules: R, table: table::Table) -> Self {
        Self {
            rules,
            table,
            started: false,
        }
    }
}

impl<R> action::Action<Game<R>> for table::Action
where
    R: for<'a> rules::Rules<table::Action, table::Table, State<'a> = GameState<'a>>,
{
    fn apply_to(self, target: &mut Game<R>) {
        assert!(target.rules.is_valid_action(target.state(), &self));
        target.table.apply(self);
    }
}
