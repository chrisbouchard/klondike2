use super::action::Actionable as _;

use super::{action, dealer, deck, rules, table};

#[derive(Debug, Clone)]
pub struct Game<D, R, S, SH>
where
    D: dealer::Dealer<table::Action>,
{
    dealer: D,
    dealer_iter: Option<D::Iter>,
    rules: R,
    settings: S,
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

impl<D, R, S, SH> Game<D, R, S, SH>
where
    D: dealer::Dealer<table::Action>,
{
    pub fn is_started(&self) -> bool {
        self.started
    }

    pub fn rules(&self) -> &R {
        &self.rules
    }

    pub fn settings(&self) -> &S {
        &self.settings
    }

    pub fn table(&self) -> &table::Table {
        &self.table
    }
}

impl<D, R, S, SH> Game<D, R, S, SH>
where
    D: dealer::Dealer<table::Action>,
    R: rules::Rules<table::Action>,
    SH: deck::Shuffle,
{
    // TODO: Possibly replace with a builder
    pub fn new(dealer: D, rules: R, settings: S, mut shuffle: SH) -> Self {
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
}

impl<D, R, RC, S, SH> action::Action<Game<D, R, S, SH>> for table::Action
where
    D: dealer::Dealer<table::Action>,
    R: for<'a> rules::Rules<table::Action, Context<'a> = RC>,
    RC: for<'a> From<&'a Game<D, R, S, SH>>,
    SH: deck::Shuffle,
{
    fn apply_to(self, target: &mut Game<D, R, S, SH>) {
        let context: RC = (target as &Game<D, R, S, SH>).into();
        assert!(target.rules.is_valid(&self, context).is_ok());
        target.table.apply(self);
    }
}

impl<D, DC, R, RC, S, SH> action::Action<Game<D, R, S, SH>> for GameAction
where
    DC: for<'a> From<&'a Game<D, R, S, SH>>,
    D: for<'a> dealer::Dealer<table::Action, Context<'a> = DC>,
    R: for<'a> rules::Rules<table::Action, Context<'a> = RC>,
    RC: for<'a> From<&'a Game<D, R, S, SH>>,
    SH: deck::Shuffle,
{
    fn apply_to(self, target: &mut Game<D, R, S, SH>) {
        match self {
            Self::Clear => {
                target.dealer_iter = None;

                let deck = deck::Deck::new_shuffled(&mut target.shuffle);
                target.table = table::Table::new_with_cards(deck);

                target.started = false;
            }
            Self::Deal => {
                let dealer = &target.dealer;

                let context: DC = (target as &Game<D, R, S, SH>).into();
                let dealer_iter = target
                    .dealer_iter
                    .get_or_insert_with(|| dealer.deal(context));

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
