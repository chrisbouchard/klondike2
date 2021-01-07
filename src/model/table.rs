use super::action;
use super::card;
use super::pile;

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
}

#[derive(Debug, Copy, Clone)]
pub enum Action {
    Deal(pile::PileId),
    Draw(usize),
    Move(pile::PileId, pile::PileId, usize),
    Reveal(pile::PileId),
}

// TODO: Use Action<Table, !> once RFC 1216 is stabilized (rust-lang/rust#35121).
impl action::Action<Table, ()> for Action {
    fn apply_to(self, table: &mut Table) -> action::Result<()> {
        match self {
            Self::Deal(target_pile_id) => {
                let dealt_card = table.stock.take_top();
                table.pile_mut(target_pile_id).place(dealt_card);
            }
            Self::Draw(count) => {
                let empty = table.stock.is_empty();

                if empty {
                    let replacement_cards = table.waste.take_all().flipped();
                    table.stock.place(replacement_cards);
                } else {
                    let drawn_cards = table.stock.take(count).flipped();
                    table.waste.place(drawn_cards);
                }
            }
            Self::Move(source_pile_id, target_pile_id, count) => {
                let moved_cards = table.pile_mut(source_pile_id).take(count);
                table.pile_mut(target_pile_id).place(moved_cards);
            }
            Self::Reveal(target_pile_id) => {
                table
                    .pile_mut(target_pile_id)
                    .flip_top_to(card::Facing::FaceUp);
            }
        }

        Ok(())
    }
}
