use super::action::Actionable as _;

use super::action;
use super::rules;
use super::table;

#[derive(Debug, Clone)]
pub struct Game {
    rules: rules::Rules,
    started: bool,
    table: table::Table,
}

#[derive(Clone, Copy, Debug)]
pub struct GameState<'a> {
    pub started: bool,
    pub table: &'a table::Table,
}

impl Game {
    // TODO: Possibly replace with a builder
    pub fn new(rules: rules::Rules, table: table::Table) -> Self {
        Self {
            rules,
            table,
            started: false,
        }
    }

    pub fn is_started(&self) -> bool {
        self.started
    }

    pub fn rules(&self) -> &rules::Rules {
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
}

impl action::Action<Game> for table::Action {
    fn apply_to(self, target: &mut Game) {
        assert!(target.rules.is_valid_action(target.state(), &self));
        target.table.apply(self);
    }
}
