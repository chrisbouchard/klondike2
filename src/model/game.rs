use std::convert;

use super::action::Actionable as _;
use super::{action, dealer, deck, rules, table};

#[derive(Debug, Clone)]
pub struct Game<D, R, S, SH, T>
where
    D: dealer::Dealer,
{
    dealer: D,
    dealer_iter: Option<D::Iter>,
    settings: S,
    shuffle: SH,
    started: bool,
    table_guard: rules::RulesGuard<R, T>,
}

#[derive(Debug, Clone)]
pub struct GameDealerContext<'a, S> {
    pub settings: &'a S,
    pub started: bool,
}

#[derive(Debug, Clone)]
pub struct GameRulesContext<'a, S, T> {
    pub settings: &'a S,
    pub started: bool,
    pub table: &'a T,
}

#[derive(Debug, Clone)]
pub enum GameAction {
    Clear,
    Start,
}

#[derive(Debug, Clone)]
pub struct TableAction<A>(A);

#[derive(Debug, Clone)]
pub struct DealAction;

impl<D, R, S, SH, T> Game<D, R, S, SH, T>
where
    D: dealer::Dealer,
{
    pub fn is_started(&self) -> bool {
        self.started
    }

    pub fn rules(&self) -> &R {
        &self.table_guard.rules()
    }

    pub fn settings(&self) -> &S {
        &self.settings
    }

    pub fn table(&self) -> &T {
        &self.table_guard.target()
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

        let table_guarded = rules::RulesGuard::new(rules, table);

        Self {
            dealer,
            settings,
            shuffle,
            table_guard: table_guarded,
            dealer_iter: None,
            started: false,
        }
    }
}

impl<D, R, S, SH, T> action::Action<Game<D, R, S, SH, T>> for GameAction
where
    D: for<'a> dealer::Dealer,
    SH: deck::Shuffle,
    T: table::Table,
{
    type Error = convert::Infallible;

    fn apply_to(self, target: &mut Game<D, R, S, SH, T>) -> Result<(), Self::Error> {
        match self {
            Self::Clear => {
                target.dealer_iter = None;

                let deck = deck::Deck::new_shuffled(&mut target.shuffle);
                target
                    .table_guard
                    .set_target(table::Table::new_with_cards(deck));

                target.started = false;
            }
            Self::Start => {
                target.started = true;
            }
        }

        Ok(())
    }
}

impl<A, D, R, RC, S, SH, T> action::Action<Game<D, R, S, SH, T>> for TableAction<A>
where
    A: action::Action<T>,
    D: dealer::Dealer,
    R: for<'a> rules::Rules<A, Context<'a> = RC>,
    RC: for<'a> From<GameRulesContext<'a, S, T>>,
    SH: deck::Shuffle,
    T: table::Table,
{
    type Error = rules::RulesGuardError<R::Error, A::Error, A>;

    fn apply_to(self, target: &mut Game<D, R, S, SH, T>) -> Result<(), Self::Error> {
        let TableAction(action) = self;
        let context = RC::from(GameRulesContext {
            settings: &target.settings,
            started: target.started,
            table: target.table_guard.target(),
        });
        target.table_guard.apply_guarded(action, &context)
    }
}

impl<A, D, DC, R, RC, S, SH, T> action::Action<Game<D, R, S, SH, T>> for DealAction
where
    A: action::Action<T>,
    D: for<'a> dealer::Dealer<Action = A, Context<'a> = DC>,
    DC: for<'a> From<GameDealerContext<'a, S>>,
    R: for<'a> rules::Rules<A, Context<'a> = RC>,
    RC: for<'a> From<GameRulesContext<'a, S, T>>,
    SH: deck::Shuffle,
    T: table::Table,
{
    type Error = rules::RulesGuardError<R::Error, A::Error, A>;

    fn apply_to(self, target: &mut Game<D, R, S, SH, T>) -> Result<(), Self::Error> {
        let dealer = &target.dealer;

        let context = DC::from(GameDealerContext {
            settings: &target.settings,
            started: target.started,
        });
        let dealer_iter = target
            .dealer_iter
            .get_or_insert_with(|| dealer.deal(context));

        if let Some(table_action) = dealer_iter.next() {
            target.apply(TableAction(table_action))?;
        }

        Ok(())
    }
}
