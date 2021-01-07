use super::action;
use super::deck;
use super::pile;
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

    pub fn table_mut(&mut self) -> &mut T {
        &mut self.table
    }

    pub fn selection(&self) -> &S {
        &self.selection
    }

    pub fn selection_mut(&mut self) -> &mut S {
        &mut self.selection
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
    pub fn with_deck(deck: deck::Deck) -> Self {
        Self {
            table: T::from(table::Table::with_stock(deck)),
            selection: S::from(selection::Selection::new()),
            started: false,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Action {
    CancelMove,
    Deal(pile::PileId),
    Draw(usize),
    GoTo(pile::PileId),
    PlaceMove,
    Reveal,
    RevealAt(pile::PileId),
    SelectLess,
    SelectMore,
    SelectAll,
    SendToFoundation,
    Start,
    TakeFromWaste,
}

impl<T, S> action::Action<Game<T, S>, ()> for Action
where
    T: action::Actionable<table::Action, ()> + AsRef<table::Table>,
    S: action::Actionable<selection::Action, ()> + AsRef<selection::Selection>,
{
    fn apply_to(self, game: &mut Game<T, S>) -> action::Result<()> {
        match self {
            Self::CancelMove => {
                game.selection.apply(selection::Action::Return)?;
            }
            Self::Deal(target_id) => {
                game.table.apply(table::Action::Deal(target_id))?;
            }
            Self::Draw(count) => {
                game.table.apply(table::Action::Draw(count))?;
            }
            Self::GoTo(target_id) => {
                game.selection.apply(selection::Action::GoTo(target_id))?;
            }
            Self::PlaceMove => {
                let source_id = game.selection.as_ref().source();
                let target_id = game.selection.as_ref().target();

                if source_id != target_id {
                    let count = game.selection.as_ref().count();
                    game.table
                        .apply(table::Action::Move(source_id, target_id, count))?;
                }

                game.selection.apply(selection::Action::Resize(0))?;
            }
            Self::Reveal => {
                let target_id = game.selection.as_ref().target();
                game.table.apply(table::Action::Reveal(target_id))?;
            }
            Self::RevealAt(target_id) => {
                game.table.apply(table::Action::Reveal(target_id))?;
            }
            Self::SelectLess => {
                let new_count = game.selection.as_ref().count().saturating_sub(1);
                game.selection.apply(selection::Action::Resize(new_count))?;
            }
            Self::SelectMore => {
                let new_count = game.selection.as_ref().count().saturating_add(1);
                game.selection.apply(selection::Action::Resize(new_count))?;
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

                game.selection.apply(selection::Action::Resize(new_count))?;
            }
            Self::SendToFoundation => {
                let source_id = game.selection.as_ref().source();
                let source_pile = game.table.as_ref().pile(source_id);

                if let Some(top_card) = source_pile.top_card() {
                    let suit = top_card.suit;
                    let target_id = pile::PileId::Foundation(suit);
                    game.table
                        .apply(table::Action::Move(source_id, target_id, 1));
                }

                let new_count = game.selection.as_ref().count().saturating_sub(1);
                game.selection.apply(selection::Action::Resize(new_count))?;
            }
            Self::Start => {
                game.started = true;
            }
            Self::TakeFromWaste => {
                // This implicitly cancels the existing selection (if any).
                game.selection
                    .apply(selection::Action::Hold(pile::PileId::Waste, 1))?;
            }
        }

        Ok(())
    }
}
