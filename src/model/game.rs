use super::action::{Action, Actionable as _};

use super::{action, dealer, deck, rules, table};

#[derive(Debug, Clone)]
pub struct Game<D, R, S, SH, T>
where
    D: dealer::Dealer,
{
    dealer: D,
    dealer_iter: Option<D::Iter>,
    rules: R,
    settings: S,
    shuffle: SH,
    started: bool,
    table: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameAction {
    Clear,
    Deal,
    Start,
}

#[derive(Debug, Clone)]
pub struct TableAction<A>(A);

impl<D, R, S, SH, T> Game<D, R, S, SH, T>
where
    D: dealer::Dealer,
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

    pub fn table(&self) -> &T {
        &self.table
    }
}

impl<D, R, S, SH, T> Game<D, R, S, SH, T>
where
    D: dealer::Dealer,
    SH: deck::Shuffle,
    T: table::Table,
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

impl<A, D, R, RC, S, SH, T> action::Action<Game<D, R, S, SH, T>> for TableAction<A>
where
    A: Action<T>,
    D: dealer::Dealer,
    R: for<'a> rules::Rules<A, Context<'a> = RC>,
    RC: for<'a> From<&'a Game<D, R, S, SH, T>>,
    SH: deck::Shuffle,
    T: table::Table,
{
    fn apply_to(self, target: &mut Game<D, R, S, SH, T>) {
        let TableAction(action) = self;
        let context: RC = (target as &Game<D, R, S, SH, T>).into();
        assert!(target.rules.is_valid(&action, context).is_ok());
        target.table.apply(action);
    }
}

impl<A, D, DC, R, RC, S, SH, T> action::Action<Game<D, R, S, SH, T>> for GameAction
where
    A: Action<T>,
    D: for<'a> dealer::Dealer<Action = A, Context<'a> = DC>,
    DC: for<'a> From<&'a Game<D, R, S, SH, T>>,
    R: for<'a> rules::Rules<A, Context<'a> = RC>,
    RC: for<'a> From<&'a Game<D, R, S, SH, T>>,
    SH: deck::Shuffle,
    T: table::Table,
{
    fn apply_to(self, target: &mut Game<D, R, S, SH, T>) {
        match self {
            Self::Clear => {
                target.dealer_iter = None;

                let deck = deck::Deck::new_shuffled(&mut target.shuffle);
                target.table = table::Table::new_with_cards(deck);

                target.started = false;
            }
            Self::Deal => {
                let dealer = &target.dealer;

                let context: DC = (target as &Game<D, R, S, SH, T>).into();
                let dealer_iter = target
                    .dealer_iter
                    .get_or_insert_with(|| dealer.deal(context));

                if let Some(table_action) = dealer_iter.next() {
                    target.apply(TableAction(table_action));
                }
            }
            Self::Start => {
                target.started = true;
            }
        }
    }
}
