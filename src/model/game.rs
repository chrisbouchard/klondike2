use super::action;
use super::action::Actionable;
use super::deck;
use super::pile;
use super::selection;
use super::table;

#[derive(Debug, Clone)]
pub struct Game {
    table: table::Table,
    selection: selection::Selection,
    started: bool,
}

impl Game {
    pub fn with_deck(deck: deck::Deck) -> Self {
        Self {
            table: table::Table::with_stock(deck),
            selection: selection::Selection::new(),
            started: false,
        }
    }

    pub fn table(&self) -> &table::Table {
        &self.table
    }

    pub fn table_mut(&mut self) -> &mut table::Table {
        &mut self.table
    }

    pub fn selection(&self) -> &selection::Selection {
        &self.selection
    }

    pub fn selection_mut(&mut self) -> &mut selection::Selection {
        &mut self.selection
    }

    pub fn is_started(&self) -> bool {
        self.started
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Action {
    CancelMove,
    // TODO: Extract Dealer from table module and refactor to use GameAction.
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

impl action::Action<Game> for Action {
    fn apply_to(self, game: &mut Game) {
        match self {
            Self::CancelMove => {
                game.selection.apply(selection::Action::Return);
            }
            Self::Deal(target_id) => {
                game.table.apply(table::Action::Deal(target_id));
            }
            Self::Draw(count) => {
                game.table.apply(table::Action::Draw(count));
            }
            Self::GoTo(target_id) => {
                game.selection.apply(selection::Action::GoTo(target_id));
            }
            Self::PlaceMove => {
                let source_id = game.selection.source();
                let target_id = game.selection.target();

                if source_id != target_id {
                    let count = game.selection.count();
                    game.table
                        .apply(table::Action::Move(source_id, target_id, count));
                }

                game.selection.apply(selection::Action::Resize(0));
            }
            Self::Reveal => {
                let target_id = game.selection.target();
                game.table.apply(table::Action::Reveal(target_id));
            }
            Self::RevealAt(target_id) => {
                game.table.apply(table::Action::Reveal(target_id));
            }
            Self::SelectLess => {
                let new_count = game.selection.count().saturating_sub(1);
                game.selection.apply(selection::Action::Resize(new_count));
            }
            Self::SelectMore => {
                let new_count = game.selection.count().saturating_add(1);
                game.selection.apply(selection::Action::Resize(new_count));
            }
            Self::SelectAll => {
                let target_id = game.selection.target();

                // Count the number of face-up cards on top of the pile. Piles iterate bottom to
                // top. Note that we don't care whether the remaining cards are all face-down.
                let new_count = {
                    game.table
                        .pile(target_id)
                        .iter()
                        .rev()
                        .take_while(|card| card.is_face_up())
                        .count()
                };

                game.selection.apply(selection::Action::Resize(new_count));
            }
            Self::SendToFoundation => {
                let source_id = game.selection.source();
                let source_pile = game.table.pile(source_id);

                if let Some(top_card) = source_pile.top_card() {
                    let suit = top_card.suit;
                    let target_id = pile::PileId::Foundation(suit);
                    game.table
                        .apply(table::Action::Move(source_id, target_id, 1));
                }

                let new_count = game.selection.count().saturating_sub(1);
                game.selection.apply(selection::Action::Resize(new_count));
            }
            Self::Start => {
                game.started = true;
            }
            Self::TakeFromWaste => {
                // This implicitly cancels the existing selection (if any).
                game.selection
                    .apply(selection::Action::Hold(pile::PileId::Waste, 1));
            }
        }
    }
}
