use super::card;
use super::pile;

#[derive(Debug, Clone)]
pub enum TableAction {
    Deal(pile::PileId),
    Draw(usize),
    Move(pile::PileId, pile::PileId, usize),
    Reveal(pile::PileId),
}

#[derive(Debug, Clone)]
pub struct TableCard<'table> {
    pub card: &'table card::Card,
    pub index: usize,
    pub pile_id: pile::PileId,
}

#[derive(Debug, Clone, Default)]
pub struct Table {
    stock: pile::Pile,
    waste: pile::Pile,

    spades_foundation: pile::Pile,
    hearts_foundation: pile::Pile,
    diamonds_foundation: pile::Pile,
    clubs_foundation: pile::Pile,

    tableaux: Vec<pile::Pile>,
    empty_tableaux: pile::Pile,
}

impl Table {
    pub fn with_stock<I>(stock: I) -> Self
    where
        I: IntoIterator<Item = card::Card>,
    {
        let mut table = Table::default();
        table.stock.place_cards(stock);
        table
    }

    pub fn pile(&self, pile_id: pile::PileId) -> &pile::Pile {
        match pile_id {
            pile::PileId::Stock => &self.stock,
            pile::PileId::Waste => &self.waste,
            pile::PileId::Foundation(suit) => match suit {
                card::Suit::Spades => &self.spades_foundation,
                card::Suit::Hearts => &self.hearts_foundation,
                card::Suit::Diamonds => &self.diamonds_foundation,
                card::Suit::Clubs => &self.clubs_foundation,
            },
            pile::PileId::Tableaux(index) => {
                self.tableaux.get(index).unwrap_or(&self.empty_tableaux)
            }
        }
    }

    pub fn pile_mut(&mut self, pile_id: pile::PileId) -> &mut pile::Pile {
        match pile_id {
            pile::PileId::Stock => &mut self.stock,
            pile::PileId::Waste => &mut self.waste,
            pile::PileId::Foundation(suit) => match suit {
                card::Suit::Spades => &mut self.spades_foundation,
                card::Suit::Hearts => &mut self.hearts_foundation,
                card::Suit::Diamonds => &mut self.diamonds_foundation,
                card::Suit::Clubs => &mut self.clubs_foundation,
            },
            pile::PileId::Tableaux(index) => {
                if index >= self.tableaux.len() {
                    self.tableaux.resize(index + 1, self.empty_tableaux.clone());
                }
                &mut self.tableaux[index]
            }
        }
    }

    pub fn cards(&self) -> impl Iterator<Item = TableCard> {
        pile::PileId::full_iter(self.tableaux.len())
            .map(move |pile_id| (pile_id, self.pile(pile_id)))
            .flat_map(|(pile_id, pile)| {
                pile.iter().enumerate().map(move |(index, card)| TableCard {
                    card,
                    index,
                    pile_id,
                })
            })
    }

    pub fn apply(&mut self, action: TableAction) {
        match action {
            TableAction::Deal(target_pile_id) => {
                let dealt_card = self.stock.take_top();
                self.pile_mut(target_pile_id).place(dealt_card);
            }
            TableAction::Draw(count) => {
                let empty = self.stock.is_empty();

                if empty {
                    let replacement_cards = self.waste.take_all().flipped();
                    self.stock.place(replacement_cards);
                } else {
                    let drawn_cards = self.stock.take(count).flipped();
                    self.waste.place(drawn_cards);
                }
            }
            TableAction::Move(source_pile_id, target_pile_id, count) => {
                let moved_cards = self.pile_mut(source_pile_id).take(count);
                self.pile_mut(target_pile_id).place(moved_cards);
            }
            TableAction::Reveal(target_pile_id) => self
                .pile_mut(target_pile_id)
                .flip_top_to(card::Facing::FaceUp),
        }
    }

    pub fn apply_all<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = TableAction>,
    {
        iterable.into_iter().for_each(|action| self.apply(action))
    }
}

#[derive(Debug)]
enum DealerState {
    Deal {
        current_index: usize,
        current_row: usize,
        width: usize,
    },
    Reveal {
        current_index: usize,
        width: usize,
    },
    Done,
}

impl DealerState {
    fn new(tableaux_width: usize) -> Self {
        Self::Deal {
            current_index: 0,
            current_row: 0,
            width: tableaux_width,
        }
    }

    fn next(&self) -> Self {
        match self {
            &Self::Deal {
                current_index,
                current_row,
                width,
            } => {
                let next_index = current_index + 1;

                if next_index < width {
                    Self::Deal {
                        current_index: next_index,
                        current_row,
                        width,
                    }
                } else {
                    let next_row = current_row + 1;

                    if next_row < width {
                        Self::Deal {
                            current_index: next_row,
                            current_row: next_row,
                            width,
                        }
                    } else {
                        Self::Reveal {
                            current_index: 0,
                            width,
                        }
                    }
                }
            }
            &Self::Reveal {
                current_index,
                width,
            } => {
                let next_index = current_index + 1;

                if next_index < width {
                    Self::Reveal {
                        current_index: next_index,
                        width,
                    }
                } else {
                    Self::Done
                }
            }
            Self::Done => Self::Done,
        }
    }
}

#[derive(Debug)]
pub struct Dealer {
    state: DealerState,
}

impl Dealer {
    pub fn new(tableaux_width: usize) -> Self {
        Dealer {
            state: DealerState::new(tableaux_width),
        }
    }
}

impl Iterator for Dealer {
    type Item = TableAction;

    fn next(&mut self) -> Option<Self::Item> {
        let action = match self.state {
            DealerState::Deal { current_index, .. } => {
                let pile_id = pile::PileId::Tableaux(current_index);
                Some(TableAction::Deal(pile_id))
            }
            DealerState::Reveal { current_index, .. } => {
                let pile_id = pile::PileId::Tableaux(current_index);
                Some(TableAction::Reveal(pile_id))
            }
            DealerState::Done => None,
        };

        self.state = self.state.next();
        action
    }
}
