use super::card;
use super::game;
use super::pile;

#[derive(Debug)]
enum DealerState {
    Deal {
        next_card: card::Card,
        current_index: usize,
        current_row: usize,
    },
    Reveal {
        current_index: usize,
    },
    Stock,
    Done,
}

impl DealerState {
    fn init<C>(card_iter: &mut C) -> Self
    where
        C: Iterator<Item = card::Card>,
    {
        match card_iter.next() {
            Some(next_card) => Self::Deal {
                next_card,
                current_index: 0,
                current_row: 0,
            },
            None => Self::Done,
        }
    }

    fn action<C>(&self, card_iter: &mut C) -> Option<game::Action>
    where
        C: Iterator<Item = card::Card>,
    {
        match self {
            &Self::Deal {
                ref next_card,
                current_index,
                ..
            } => {
                let pile_id = pile::PileId::Tableaux(current_index);
                Some(game::Action::Deal(pile_id, next_card.clone()))
            }
            &Self::Reveal { current_index, .. } => {
                let pile_id = pile::PileId::Tableaux(current_index);
                Some(game::Action::RevealAt(pile_id))
            }
            Self::Stock => Some(game::Action::Stock(card_iter.collect())),
            Self::Done => None,
        }
    }

    fn next<C>(&self, tableaux_width: usize, card_iter: &mut C) -> Self
    where
        C: Iterator<Item = card::Card>,
    {
        match self {
            &Self::Deal {
                current_index,
                current_row,
                ..
            } => {
                let maybe_next_index_and_row = {
                    let next_index = current_index + 1;

                    if next_index < tableaux_width {
                        Some((next_index, current_row))
                    } else {
                        let next_row = current_row + 1;

                        if next_row < tableaux_width {
                            Some((next_row, next_row))
                        } else {
                            None
                        }
                    }
                };

                maybe_next_index_and_row
                    .and_then(|(next_index, next_row)| {
                        card_iter.next().map(|next_card| Self::Deal {
                            next_card,
                            current_index: next_index,
                            current_row: next_row,
                        })
                    })
                    .unwrap_or(Self::Reveal { current_index: 0 })
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
            Self::Stock => Self::Done,
            Self::Done => Self::Done,
        }
    }
}

#[derive(Debug)]
pub struct Dealer<C> {
    card_iter: C,
    state: DealerState,
    tableaux_width: usize,
}

impl<C> Dealer<C>
where
    C: Iterator<Item = card::Card>,
{
    pub fn with_cards<I>(cards: I) -> Self
    where
        I: IntoIterator<Item = card::Card, IntoIter = C>,
    {
        let mut card_iter = cards.into_iter();
        let state = DealerState::init(&mut card_iter);

        Self {
            card_iter,
            state,
            tableaux_width: 0,
        }
    }
}

impl<C> Iterator for Dealer<C>
where
    C: Iterator<Item = card::Card>,
{
    type Item = game::Action;

    fn next(&mut self) -> Option<Self::Item> {
        let action = self.state.action(&mut self.card_iter);
        self.state = self.state.next(self.tableaux_width, &mut self.card_iter);
        action
    }
}
