use super::action::Actionable as _;

use super::action;
use super::dealer;
use super::deck;
use super::rules;
use super::settings;
use super::table;

#[derive(Debug)]
pub struct Game<S> {
    dealer: dealer::Dealer,
    dealer_iter: dealer::Iter,
    rules: rules::Rules,
    settings: settings::Settings,
    shuffle: S,
    started: bool,
    table: table::Table,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameAction {
    Clear,
    Deal,
    Start,
}

impl<S> Game<S>
where
    S: deck::Shuffle,
{
    // TODO: Possibly replace with a builder
    pub fn new(
        dealer: dealer::Dealer,
        rules: rules::Rules,
        settings: settings::Settings,
        mut shuffle: S,
    ) -> Self {
        let dealer_iter = dealer.deal_cards(settings.tableaux_width);

        let deck = deck::Deck::new_shuffled(&mut shuffle);
        let table = table::Table::new_with_cards(deck);

        Self {
            dealer,
            dealer_iter,
            rules,
            settings,
            shuffle,
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

    pub fn settings(&self) -> &settings::Settings {
        &self.settings
    }

    pub fn table(&self) -> &table::Table {
        &self.table
    }
}

impl<S> action::Action<Game<S>> for table::Action
where
    S: deck::Shuffle,
{
    fn apply_to(self, target: &mut Game<S>) {
        let state = rules::RuleState {
            settings: &target.settings,
            started: target.is_started(),
            table: &target.table,
        };
        assert!(target.rules.is_valid_action(state, &self));
        target.table.apply(self);
    }
}

impl<S> action::Action<Game<S>> for GameAction
where
    S: deck::Shuffle,
{
    fn apply_to(self, target: &mut Game<S>) {
        match self {
            Self::Clear => {
                target.dealer_iter = target.dealer.deal_cards(target.settings.tableaux_width);

                let deck = deck::Deck::new_shuffled(&mut target.shuffle);
                target.table = table::Table::new_with_cards(deck);

                target.started = false;
            }
            Self::Deal => {
                if let Some(table_action) = target.dealer_iter.next() {
                    target.apply(table_action);
                }
            }
            Self::Start => {
                target.started = true;
            }
        }
    }
}
