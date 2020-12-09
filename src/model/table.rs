use super::action;
use super::card;
use super::pile;

#[derive(Debug, Copy, Clone)]
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
}

impl Table {
    pub fn with_stock<I>(stock: I) -> Self
    where
        I: IntoIterator<Item = card::Card>,
    {
        let mut table = Self::default();
        table.stock.place_cards(stock);
        table
    }

    pub fn pile(&self, pile_id: pile::PileId) -> &pile::Pile {
        static EMPTY: pile::Pile = pile::Pile::new();

        match pile_id {
            pile::PileId::Stock => &self.stock,
            pile::PileId::Waste => &self.waste,
            pile::PileId::Foundation(suit) => match suit {
                card::Suit::Spades => &self.spades_foundation,
                card::Suit::Hearts => &self.hearts_foundation,
                card::Suit::Diamonds => &self.diamonds_foundation,
                card::Suit::Clubs => &self.clubs_foundation,
            },
            pile::PileId::Tableaux(index) => self.tableaux.get(index).unwrap_or(&EMPTY),
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
                    self.tableaux.resize_with(index + 1, Default::default);
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
}

impl action::Actionable for Table {
    type Action = TableAction;

    fn apply(&mut self, action: Self::Action) {
        match action {
            Self::Action::Deal(target_pile_id) => {
                let dealt_card = self.stock.take_top();
                self.pile_mut(target_pile_id).place(dealt_card);
            }
            Self::Action::Draw(count) => {
                let empty = self.stock.is_empty();

                if empty {
                    let replacement_cards = self.waste.take_all().flipped();
                    self.stock.place(replacement_cards);
                } else {
                    let drawn_cards = self.stock.take(count).flipped();
                    self.waste.place(drawn_cards);
                }
            }
            Self::Action::Move(source_pile_id, target_pile_id, count) => {
                let moved_cards = self.pile_mut(source_pile_id).take(count);
                self.pile_mut(target_pile_id).place(moved_cards);
            }
            Self::Action::Reveal(target_pile_id) => self
                .pile_mut(target_pile_id)
                .flip_top_to(card::Facing::FaceUp),
        }
    }
}

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

        self.state = self.state.next(self.tableaux_width);
        action
    }
}
