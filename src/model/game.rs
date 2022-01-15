use super::action::Actionable as _;

use super::{action, dealer, deck, rules, settings, table};

#[derive(Debug, Clone)]
pub struct Game<D, SH>
where
    D: dealer::Dealer<table::Action>,
{
    dealer: D,
    dealer_iter: Option<D::Iter>,
    rules: rules::Rules,
    settings: settings::Settings,
    shuffle: SH,
    started: bool,
    table: table::Table,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameAction {
    Clear,
    Deal,
    Start,
}

impl<'a, D, SH> Game<D, SH>
where
    D: dealer::Dealer<table::Action>,
    SH: deck::Shuffle,
{
    // TODO: Possibly replace with a builder
    pub fn new(
        dealer: D,
        rules: rules::Rules,
        settings: settings::Settings,
        mut shuffle: SH,
    ) -> Self {
        let deck = deck::Deck::new_shuffled(&mut shuffle);
        let table = table::Table::new_with_cards(deck);

        Self {
            dealer,
            rules,
            settings,
            shuffle,
            table,
            dealer_iter: None,
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

impl<D, SH> action::Action<Game<D, SH>> for table::Action
where
    D: dealer::Dealer<table::Action>,
    SH: deck::Shuffle,
{
    fn apply_to(self, target: &mut Game<D, SH>) {
        let state = rules::RuleState {
            settings: &target.settings,
            started: target.is_started(),
            table: &target.table,
        };
        assert!(target.rules.is_valid_action(state, &self));
        target.table.apply(self);
    }
}

impl<C, D, SH> action::Action<Game<D, SH>> for GameAction
where
    C: for<'a> From<&'a Game<D, SH>>,
    D: for<'a> dealer::Dealer<table::Action, Context<'a> = C>,
    SH: deck::Shuffle,
{
    fn apply_to(self, target: &mut Game<D, SH>) {
        match self {
            Self::Clear => {
                target.dealer_iter = None;

                let deck = deck::Deck::new_shuffled(&mut target.shuffle);
                target.table = table::Table::new_with_cards(deck);

                target.started = false;
            }
            Self::Deal => {
                let dealer = &target.dealer;

                // We need to get a new dealer ready, because we can't borrow
                // target.dealer while also mutably borrowing target.dealer_iter
                // to call get_or_insert_with.
                let new_dealer_iter = dealer.deal((target as &Game<D, SH>).into());
                let dealer_iter = target.dealer_iter.get_or_insert(new_dealer_iter);

                if let Some(table_action) = dealer_iter.next() {
                    target.apply(table_action);
                }
            }
            Self::Start => {
                target.started = true;
            }
        }
    }
}
