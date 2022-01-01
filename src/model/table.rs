use enum_like::EnumValues as _;
use snafu;

use super::action;
use super::card;
use super::pile;
use super::rules;

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
    AddStock(pile::Pile),
    Deal(PileId, card::Card),
    Draw(usize),
    Move(PileId, PileId, usize),
    Reveal(PileId),
}

impl action::Action<Table> for Action {
    // TODO: Use Error = ! once RFC 1216 is stabilized (rust-lang/rust#35121).
    type Error = std::convert::Infallible;

    fn apply_to(self, table: &mut Table) -> Result<(), Self::Error> {
        match self {
            Self::AddStock(pile) => {
                table.stock.place(pile);
            }
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
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Rules {
    can_move_from_foundation: bool,
    tableaux_width: usize,
}

#[derive(Debug, Copy, Clone, derive_more::Display)]
pub enum FoundationMismatchType {
    #[display("")]
    Start(card::Suit),
    #[display("")]
    Follow(card::CardFace),
}

#[derive(Debug, Copy, Clone, derive_more::Display)]
pub enum TableauxMismatchType {
    #[display("")]
    Start,
    #[display("")]
    Follow(card::CardFace),
}

#[derive(Debug, snafu::Snafu)]
pub enum RulesError {
    #[snafu(display(""))]
    EmptyMove,
    #[snafu(display(""))]
    FoundationMismatch {
        card: card::CardFace,
        mismatch: FoundationMismatchType,
    },
    #[snafu(display(""))]
    IllegalMoveFromFoundation,
    #[snafu(display(""))]
    InsufficientCards,
    #[snafu(display(""))]
    InvalidDealTarget { pile_id: PileId },
    #[snafu(display(""))]
    InvalidFacing { card: card::CardFace },
    #[snafu(display(""))]
    InvalidMoveSource { pile_id: PileId },
    #[snafu(display(""))]
    InvalidMoveTarget { pile_id: PileId },
    #[snafu(display(""))]
    MayOnlyAcceptSingleCard { pile_id: PileId },
    #[snafu(display(""))]
    MayOnlyTakeSingleCard { pile_id: PileId },
    #[snafu(display(""))]
    PileOutOfBounds { index: usize },
    #[snafu(display(""))]
    TableauxMismatch {
        card: card::CardFace,
        mismatch: TableauxMismatchType,
    },
}

impl rules::Rules<Table, Action> for Rules {
    type Error = RulesError;

    fn check_rules(&self, target: &Table, action: &Action) -> Result<(), Self::Error> {
        match action {
            &Action::Deal(target_pile_id, ref dealt_card) => {
                if let PileId::Tableaux(index) = target_pile_id {
                    snafu::ensure!(index < self.tableaux_width, PileOutOfBounds { index });

                    if let Some(top_card) = target.pile(target_pile_id).top_card() {
                        snafu::ensure!(
                            top_card.facing == card::Facing::FaceDown,
                            InvalidFacing {
                                card: dealt_card.face
                            }
                        );
                    }

                    Ok(())
                } else {
                    Err(RulesError::InvalidDealTarget {
                        pile_id: target_pile_id,
                    })
                }
            }
            &Action::Draw(..) => {
                //TODO: Should we check this?
                Ok(())
            }
            &Action::Move(source_pile_id, target_pile_id, count) => {
                snafu::ensure!(count > 0, EmptyMove);

                let source_top_cards = target.pile(source_pile_id).top_cards(count);
                snafu::ensure!(count == source_top_cards.len(), InsufficientCards);

                // We assume it's sufficient to check the facing of the bottom card of the pile.
                snafu::ensure!(
                    source_top_cards[0].is_face_up(),
                    InvalidFacing {
                        card: source_top_cards[0].face
                    }
                );

                match source_pile_id {
                    PileId::Tableaux(index) => {
                        snafu::ensure!(index < self.tableaux_width, PileOutOfBounds { index });
                    }
                    PileId::Foundation(_) => {
                        snafu::ensure!(self.can_move_from_foundation, IllegalMoveFromFoundation);
                        snafu::ensure!(
                            count == 1,
                            MayOnlyTakeSingleCard {
                                pile_id: source_pile_id
                            }
                        );
                    }
                    PileId::Waste => {
                        snafu::ensure!(
                            count == 1,
                            MayOnlyTakeSingleCard {
                                pile_id: source_pile_id
                            }
                        );
                    }
                    _ => {
                        return Err(RulesError::InvalidMoveSource {
                            pile_id: source_pile_id,
                        });
                    }
                }

                match target_pile_id {
                    PileId::Tableaux(index) => {
                        snafu::ensure!(index < self.tableaux_width, PileOutOfBounds { index });

                        if let Some(target_top_card) = target.pile(target_pile_id).top_card() {
                            snafu::ensure!(
                                target_top_card.rank().follows(source_top_cards[0].rank()),
                                TableauxMismatch {
                                    card: source_top_cards[0].face,
                                    mismatch: TableauxMismatchType::Follow(target_top_card.face)
                                }
                            );
                            snafu::ensure!(
                                target_top_card.color() != source_top_cards[0].color(),
                                TableauxMismatch {
                                    card: target_top_card.face,
                                    mismatch: TableauxMismatchType::Follow(target_top_card.face)
                                }
                            );
                        } else {
                            snafu::ensure!(
                                source_top_cards[0].is_king(),
                                TableauxMismatch {
                                    card: source_top_cards[0].face,
                                    mismatch: TableauxMismatchType::Start
                                }
                            );
                        }
                    }
                    PileId::Foundation(suit) => {
                        snafu::ensure!(
                            count == 1,
                            MayOnlyAcceptSingleCard {
                                pile_id: target_pile_id,
                            }
                        );

                        // We can expect this because we checked the source had sufficient cards
                        // above.
                        let source_card = &source_top_cards[0];

                        if let Some(target_top_card) = target.pile(target_pile_id).top_card() {
                            snafu::ensure!(
                                target_top_card.color() == source_card.color(),
                                FoundationMismatch {
                                    card: target_top_card.face,
                                    mismatch: FoundationMismatchType::Follow(target_top_card.face)
                                }
                            );
                            snafu::ensure!(
                                target_top_card.rank().follows(source_card.rank()),
                                FoundationMismatch {
                                    card: source_card.face,
                                    mismatch: FoundationMismatchType::Follow(target_top_card.face)
                                }
                            );
                        } else {
                            snafu::ensure!(
                                source_card.suit() == suit,
                                FoundationMismatch {
                                    card: source_card.face,
                                    mismatch: FoundationMismatchType::Start(suit)
                                }
                            );
                            snafu::ensure!(
                                source_card.is_ace(),
                                FoundationMismatch {
                                    card: source_card.face,
                                    mismatch: FoundationMismatchType::Start(suit)
                                }
                            );
                        }
                    }
                    _ => {
                        return Err(RulesError::InvalidMoveTarget {
                            pile_id: target_pile_id,
                        });
                    }
                }

                Ok(())
            }
            &Action::AddStock(ref pile) => todo!(),
            &Action::Reveal(target_pile_id) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test]
    fn new_should_not_panic() {
        Table::new();
    }

    #[test_case(PileId::Stock => true; "Stock")]
    #[test_case(PileId::Waste => true; "Waste")]
    #[test_case(PileId::Foundation(card::Suit::Spades) => true; "Spades Foundation")]
    #[test_case(PileId::Foundation(card::Suit::Hearts) => true; "Hearts Foundation")]
    #[test_case(PileId::Foundation(card::Suit::Diamonds) => true; "Diamonds Foundation")]
    #[test_case(PileId::Foundation(card::Suit::Clubs) => true; "Clubs Foundation")]
    #[test_case(PileId::Tableaux(0) => true; "Tableaux Index 0")]
    #[test_case(PileId::Tableaux(6) => true; "Tableaux Index 6")]
    #[test_case(PileId::Tableaux(99) => true; "Tableaux Index 99")]
    fn new_should_create_empty_table(pile_id: PileId) -> bool {
        let table = Table::new();
        table.pile(pile_id).is_empty()
    }
}
