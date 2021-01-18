use enum_like::EnumValues as _;

use super::action;
use super::card;
use super::pile;

#[derive(Debug, Copy, Clone, Eq, PartialEq, derive_more::Display)]
pub enum PileId {
    #[display(fmt = "Stock")]
    Stock,
    #[display(fmt = "Waste")]
    Waste,
    #[display(fmt = "{} Foundation", _0)]
    Foundation(card::Suit),
    #[display(fmt = "Tableaux {}", _0 + 1)]
    Tableaux(usize),
}

impl PileId {
    pub fn standard_iter() -> impl Iterator<Item = PileId> {
        velcro::iter![
            PileId::Stock,
            PileId::Waste,
            ..card::Suit::values().map(PileId::Foundation)
        ]
    }

    pub fn full_iter(tableaux_width: usize) -> impl Iterator<Item = PileId> {
        velcro::iter![
            ..Self::standard_iter(),
            ..(0..tableaux_width).map(PileId::Tableaux)
        ]
    }
}

#[derive(Debug, Clone)]
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
    pub const fn new() -> Self {
        Self {
            stock: pile::Pile::new(),
            waste: pile::Pile::new(),

            spades_foundation: pile::Pile::new(),
            hearts_foundation: pile::Pile::new(),
            diamonds_foundation: pile::Pile::new(),
            clubs_foundation: pile::Pile::new(),

            tableaux: Vec::new(),
        }
    }

    pub fn pile(&self, pile_id: PileId) -> &pile::Pile {
        static EMPTY: pile::Pile = pile::Pile::new();

        match pile_id {
            PileId::Stock => &self.stock,
            PileId::Waste => &self.waste,
            PileId::Foundation(suit) => match suit {
                card::Suit::Spades => &self.spades_foundation,
                card::Suit::Hearts => &self.hearts_foundation,
                card::Suit::Diamonds => &self.diamonds_foundation,
                card::Suit::Clubs => &self.clubs_foundation,
            },
            PileId::Tableaux(index) => self.tableaux.get(index).unwrap_or(&EMPTY),
        }
    }

    fn pile_mut(&mut self, pile_id: PileId) -> &mut pile::Pile {
        match pile_id {
            PileId::Stock => &mut self.stock,
            PileId::Waste => &mut self.waste,
            PileId::Foundation(suit) => match suit {
                card::Suit::Spades => &mut self.spades_foundation,
                card::Suit::Hearts => &mut self.hearts_foundation,
                card::Suit::Diamonds => &mut self.diamonds_foundation,
                card::Suit::Clubs => &mut self.clubs_foundation,
            },
            PileId::Tableaux(index) => {
                if index >= self.tableaux.len() {
                    self.tableaux.resize_with(index + 1, Default::default);
                }
                &mut self.tableaux[index]
            }
        }
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum Action {
    Deal(PileId, card::Card),
    Draw(usize),
    Move(PileId, PileId, usize),
    Reveal(PileId),
    Stock(Vec<card::Card>),
}

#[derive(Debug, snafu::Snafu)]
pub enum Error {}

impl action::Action<Table> for Action {
    // TODO: Use Error = ! once RFC 1216 is stabilized (rust-lang/rust#35121).
    type Error = Error;

    fn apply_to(self, table: &mut Table) -> Result<(), Self::Error> {
        match self {
            Self::Deal(target_pile_id, card) => {
                table.pile_mut(target_pile_id).place_one(card);
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
            Self::Stock(stock) => {
                table.stock.place_cards(stock);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_not_panic() {
        Table::new();
    }

    #[test]
    fn new_should_create_empty_stock() {
        let table = Table::new();
        assert!(table.pile(PileId::Stock).is_empty(), "Stock is not empty");
    }

    #[test]
    fn new_should_create_empty_waste() {
        let table = Table::new();
        assert!(table.pile(PileId::Waste).is_empty(), "Waste is not empty");
    }

    #[test]
    fn new_should_create_empty_spades_foundation() {
        let table = Table::new();
        assert!(
            table
                .pile(PileId::Foundation(card::Suit::Spades))
                .is_empty(),
            "Spades foundation is not empty"
        );
    }

    #[test]
    fn new_should_create_empty_hearts_foundation() {
        let table = Table::new();
        assert!(
            table
                .pile(PileId::Foundation(card::Suit::Hearts))
                .is_empty(),
            "Hearts foundation is not empty"
        );
    }

    #[test]
    fn new_should_create_empty_diamonds_foundation() {
        let table = Table::new();
        assert!(
            table
                .pile(PileId::Foundation(card::Suit::Diamonds))
                .is_empty(),
            "Diamonds foundation is not empty"
        );
    }

    #[test]
    fn new_should_create_empty_clubs_foundation() {
        let table = Table::new();
        assert!(
            table.pile(PileId::Foundation(card::Suit::Clubs)).is_empty(),
            "Clubs foundation is not empty"
        );
    }

    #[test]
    fn new_should_create_empty_tableaux() {
        let table = Table::new();
        assert!(
            table.pile(PileId::Tableaux(0)).is_empty(),
            "Tableaux is not empty"
        );
    }
}
