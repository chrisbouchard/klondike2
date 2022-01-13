use super::action::Actionable as _;

use super::action;
use super::deck;
use super::rules;
use super::table;

#[derive(Debug, Clone)]
pub struct Game {
    rules: rules::Rules,
    started: bool,
    table: table::Table,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameAction {
    Clear,
    Start,
}

impl Game {
    // TODO: Possibly replace with a builder
    pub fn new(rules: rules::Rules, shuffle: &mut dyn deck::Shuffle) -> Self {
        let deck = deck::Deck::new_shuffled(shuffle);
        let table = table::Table::new_with_cards(deck);

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

    pub fn table(&self) -> &table::Table {
        &self.table
    }

    pub fn restart(&mut self, shuffle: &mut dyn deck::Shuffle) {
        let deck = deck::Deck::new_shuffled(shuffle);
        self.table = table::Table::new_with_cards(deck);
    }
}

impl action::Action<Game> for table::Action {
    fn apply_to(self, target: &mut Game) {
        let state = rules::RuleState {
            started: target.is_started(),
            table: target.table(),
        };
        assert!(target.rules.is_valid_action(state, &self));
        target.table.apply(self);
    }
}

impl action::Action<Game> for GameAction {
    fn apply_to(self, target: &mut Game) {
        match self {
            Self::Clear => {
                // TODO: Use a shuffled deck.
                let deck = deck::Deck::new();
                target.table = table::Table::new_with_cards(deck);
                target.started = false;
            }
            Self::Start => {
                target.started = true;
            }
        }
    }
}
