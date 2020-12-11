use super::action;
use super::deck;
use super::pile;
use super::selection;
use super::table;

#[derive(Debug, Copy, Clone)]
pub enum Action {
    CancelMove,
    // TODO: Extract Dealer from table module and refactor to use GameAction.
    Deal(pile::PileId),
    Draw(usize),
    GoTo(pile::PileId),
    PlaceMove,
    Reveal,
    SelectLess,
    SelectMore,
    SelectAll,
    SendToFoundation,
    Start,
    TakeFromWaste,
}

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

impl action::Actionable for Game {
    type Action = Action;

    fn apply(&mut self, action: Action) {
        match action {
            Self::Action::CancelMove => {
                self.selection.apply(selection::Action::Cancel);
            }
            Self::Action::Deal(target_id) => {
                self.table.apply(table::Action::Deal(target_id));
            }
            Self::Action::Draw(count) => {
                self.table.apply(table::Action::Draw(count));
            }
            Self::Action::GoTo(target_id) => {
                self.selection.apply(selection::Action::Move(target_id));
            }
            Self::Action::PlaceMove => {
                let source_id = self.selection.source();
                let target_id = self.selection.target();

                if source_id != target_id {
                    let count = self.selection.count();
                    self.table
                        .apply(table::Action::Move(source_id, target_id, count));
                }

                self.selection.apply(selection::Action::Place);
            }
            Self::Action::Reveal => {
                let target_id = self.selection.target();
                self.table.apply(table::Action::Reveal(target_id));
            }
            Self::Action::SelectLess => {
                let new_count = self.selection.count().saturating_sub(1);
                self.selection.apply(selection::Action::Resize(new_count));
            }
            Self::Action::SelectMore => {
                let new_count = self.selection.count().saturating_add(1);
                self.selection.apply(selection::Action::Resize(new_count));
            }
            Self::Action::SelectAll => {
                let target_id = self.selection.target();

                // Count the number of face-up cards on top of the pile. Piles iterate bottom to
                // top. Note that we don't care whether the remaining cards are all face-down.
                let new_count = {
                    self.table
                        .pile(target_id)
                        .iter()
                        .rev()
                        .take_while(|card| card.is_face_up())
                        .count()
                };

                self.selection.apply(selection::Action::Resize(new_count));
            }
            Self::Action::SendToFoundation => {
                let source_id = self.selection.source();
                let source_pile = self.table.pile(source_id);

                if let Some(top_card) = source_pile.top_card() {
                    let suit = top_card.suit;
                    let target_id = pile::PileId::Foundation(suit);
                    self.table
                        .apply(table::Action::Move(source_id, target_id, 1));
                }

                self.selection.apply(selection::Action::Cancel);
            }
            Self::Action::Start => {
                self.started = true;
            }
            Self::Action::TakeFromWaste => {
                // This implicitly cancels the existing selection (if any).
                self.selection
                    .apply(selection::Action::Replace(pile::PileId::Waste, 1));
            }
        }
    }
}
