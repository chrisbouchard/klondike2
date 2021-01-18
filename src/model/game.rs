use std::error;

use snafu::ResultExt as _;

use super::action;
use super::card;
use super::selection;
use super::table;

#[derive(Debug, Clone)]
pub struct Game<T, S> {
    table: T,
    selection: S,
    started: bool,
}

impl<T, S> Game<T, S> {
    pub fn table(&self) -> &T {
        &self.table
    }

    pub fn selection(&self) -> &S {
        &self.selection
    }

    pub fn is_started(&self) -> bool {
        self.started
    }
}

impl<T, S> Game<T, S>
where
    T: From<table::Table>,
    S: From<selection::Selection>,
{
    // TODO: Possibly replace with a builder
    pub fn new() -> Self {
        Self {
            table: T::from(table::Table::new()),
            selection: S::from(selection::Selection::new()),
            started: false,
        }
    }
}

impl<T, S> Default for Game<T, S>
where
    T: From<table::Table>,
    S: From<selection::Selection>,
{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum Action {
    CancelMove,
    Deal(table::PileId, card::Card),
    Draw(usize),
    GoTo(table::PileId),
    PlaceMove,
    Reveal,
    RevealAt(table::PileId),
    SelectLess,
    SelectMore,
    SelectAll,
    SendToFoundation,
    Start,
    Stock(Vec<card::Card>),
    TakeFromWaste,
}

#[derive(Debug, snafu::Snafu)]
pub enum Error<T, S>
where
    T: error::Error + 'static,
    S: error::Error + 'static,
{
    TableError { source: T, action: Action },
    SelectionError { source: S, action: Action },
}

impl<T, S> action::Action<Game<T, S>> for Action
where
    T: action::Actionable<table::Action> + AsRef<table::Table>,
    S: action::Actionable<selection::Action> + AsRef<selection::Selection>,
    T::Error: error::Error + 'static,
    S::Error: error::Error + 'static,
{
    type Error = Error<T::Error, S::Error>;

    fn apply_to(self, game: &mut Game<T, S>) -> Result<(), Self::Error> {
        match self {
            Self::CancelMove => {
                game.selection
                    .apply(selection::Action::Return)
                    .context(SelectionError { action: self })?;
            }
            Self::Deal(target_id, ref card) => {
                game.table
                    .apply(table::Action::Deal(target_id, card.clone()))
                    .context(TableError { action: self })?;
            }
            Self::Draw(count) => {
                game.table
                    .apply(table::Action::Draw(count))
                    .context(TableError { action: self })?;
            }
            Self::GoTo(target_id) => {
                game.selection
                    .apply(selection::Action::GoTo(target_id))
                    .context(SelectionError { action: self })?;
            }
            Self::PlaceMove => {
                let source_id = game.selection.as_ref().source();
                let target_id = game.selection.as_ref().target();

                if source_id != target_id {
                    let count = game.selection.as_ref().count();
                    game.table
                        .apply(table::Action::Move(source_id, target_id, count))
                        .context(TableError {
                            action: self.clone(),
                        })?;
                }

                game.selection
                    .apply(selection::Action::Resize(0))
                    .context(SelectionError { action: self })?;
            }
            Self::Reveal => {
                let target_id = game.selection.as_ref().target();
                game.table
                    .apply(table::Action::Reveal(target_id))
                    .context(TableError { action: self })?;
            }
            Self::RevealAt(target_id) => {
                game.table
                    .apply(table::Action::Reveal(target_id))
                    .context(TableError { action: self })?;
            }
            Self::SelectLess => {
                let new_count = game.selection.as_ref().count().saturating_sub(1);
                game.selection
                    .apply(selection::Action::Resize(new_count))
                    .context(SelectionError { action: self })?;
            }
            Self::SelectMore => {
                let new_count = game.selection.as_ref().count().saturating_add(1);
                game.selection
                    .apply(selection::Action::Resize(new_count))
                    .context(SelectionError { action: self })?;
            }
            Self::SelectAll => {
                let target_id = game.selection.as_ref().target();

                // Count the number of face-up cards on top of the pile. Piles iterate bottom to
                // top. Note that we don't care whether the remaining cards are all face-down.
                let new_count = {
                    game.table
                        .as_ref()
                        .pile(target_id)
                        .iter()
                        .rev()
                        .take_while(|card| card.is_face_up())
                        .count()
                };

                game.selection
                    .apply(selection::Action::Resize(new_count))
                    .context(SelectionError { action: self })?;
            }
            Self::SendToFoundation => {
                let source_id = game.selection.as_ref().source();
                let source_pile = game.table.as_ref().pile(source_id);

                if let Some(top_card) = source_pile.top_card() {
                    let suit = top_card.suit();
                    let target_id = table::PileId::Foundation(suit);
                    game.table
                        .apply(table::Action::Move(source_id, target_id, 1))
                        .context(TableError {
                            action: self.clone(),
                        })?;
                }

                let new_count = game.selection.as_ref().count().saturating_sub(1);
                game.selection
                    .apply(selection::Action::Resize(new_count))
                    .context(SelectionError { action: self })?;
            }
            Self::Start => {
                game.started = true;
            }
            Self::Stock(ref stock) => {
                game.table
                    .apply(table::Action::Stock(stock.clone()))
                    .context(TableError { action: self })?;
            }
            Self::TakeFromWaste => {
                // This implicitly cancels the existing selection (if any).
                game.selection
                    .apply(selection::Action::Hold(table::PileId::Waste, 1))
                    .context(SelectionError { action: self })?;
            }
        }

        Ok(())
    }
}
