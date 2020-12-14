use super::game;
use super::pile;

#[derive(Debug)]
enum DealerState {
    Deal {
        current_index: usize,
        current_row: usize,
    },
    Reveal {
        current_index: usize,
    },
    Done,
}

impl DealerState {
    const fn init() -> Self {
        Self::Deal {
            current_index: 0,
            current_row: 0,
        }
    }

    fn action(&self) -> Option<game::Action> {
        match self {
            &Self::Deal { current_index, .. } => {
                let pile_id = pile::PileId::Tableaux(current_index);
                Some(game::Action::Deal(pile_id))
            }
            &Self::Reveal { current_index, .. } => {
                let pile_id = pile::PileId::Tableaux(current_index);
                Some(game::Action::RevealAt(pile_id))
            }
            Self::Done => None,
        }
    }

    fn next(&self, tableaux_width: usize) -> Self {
        match self {
            &Self::Deal {
                current_index,
                current_row,
            } => {
                let next_index = current_index + 1;

                if next_index < tableaux_width {
                    Self::Deal {
                        current_index: next_index,
                        current_row,
                    }
                } else {
                    let next_row = current_row + 1;

                    if next_row < tableaux_width {
                        Self::Deal {
                            current_index: next_row,
                            current_row: next_row,
                        }
                    } else {
                        Self::Reveal { current_index: 0 }
                    }
                }
            }
            &Self::Reveal { current_index } => {
                let next_index = current_index + 1;

                if next_index < tableaux_width {
                    Self::Reveal {
                        current_index: next_index,
                    }
                } else {
                    Self::Done
                }
            }
            Self::Done => Self::Done,
        }
    }
}

impl Default for DealerState {
    fn default() -> Self {
        Self::init()
    }
}

#[derive(Debug, Default)]
pub struct Dealer {
    state: DealerState,
    tableaux_width: usize,
}

impl Dealer {
    pub const fn new() -> Self {
        Self {
            state: DealerState::init(),
            tableaux_width: 0,
        }
    }
}

impl Iterator for Dealer {
    type Item = game::Action;

    fn next(&mut self) -> Option<Self::Item> {
        let action = self.state.action();
        self.state = self.state.next(self.tableaux_width);
        action
    }
}
