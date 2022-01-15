use enum_like::EnumValues as _;

use crate::model;

#[derive(Debug, Copy, Clone, Eq, PartialEq, derive_more::Display)]
pub enum KlondikePileId {
    #[display(fmt = "Stock")]
    Stock,
    #[display(fmt = "Waste")]
    Waste,
    #[display(fmt = "{} Foundation", _0)]
    Foundation(model::card::Suit),
    #[display(fmt = "Tableaux {}", _0 + 1)]
    Tableaux(usize),
}

impl KlondikePileId {
    pub fn standard_iter() -> impl Iterator<Item = KlondikePileId> {
        velcro::iter![
            KlondikePileId::Stock,
            KlondikePileId::Waste,
            ..model::card::Suit::values().map(KlondikePileId::Foundation)
        ]
    }

    pub fn full_iter(tableaux_width: usize) -> impl Iterator<Item = KlondikePileId> {
        velcro::iter![
            ..Self::standard_iter(),
            ..(0..tableaux_width).map(KlondikePileId::Tableaux)
        ]
    }
}

#[derive(Debug, Default, Clone)]
pub struct KlondikeTable {
    stock: model::pile::Pile,
    waste: model::pile::Pile,

    spades_foundation: model::pile::Pile,
    hearts_foundation: model::pile::Pile,
    diamonds_foundation: model::pile::Pile,
    clubs_foundation: model::pile::Pile,

    tableaux: Vec<model::pile::Pile>,
}

impl model::table::Table for KlondikeTable {
    type PileId = KlondikePileId;

    fn new_with_cards<I>(cards: I) -> Self
    where
        I: IntoIterator<Item = model::card::Card>,
    {
        let stock = model::pile::Pile::new_with_cards(cards);
        assert!(stock.is_face_down());

        Self {
            stock,
            waste: model::pile::Pile::new(),

            spades_foundation: model::pile::Pile::new(),
            hearts_foundation: model::pile::Pile::new(),
            diamonds_foundation: model::pile::Pile::new(),
            clubs_foundation: model::pile::Pile::new(),

            tableaux: Vec::new(),
        }
    }

    fn pile(&self, pile_id: KlondikePileId) -> &model::pile::Pile {
        static EMPTY: model::pile::Pile = model::pile::Pile::new();

        match pile_id {
            KlondikePileId::Stock => &self.stock,
            KlondikePileId::Waste => &self.waste,
            KlondikePileId::Foundation(suit) => match suit {
                model::card::Suit::Spades => &self.spades_foundation,
                model::card::Suit::Hearts => &self.hearts_foundation,
                model::card::Suit::Diamonds => &self.diamonds_foundation,
                model::card::Suit::Clubs => &self.clubs_foundation,
            },
            KlondikePileId::Tableaux(index) => self.tableaux.get(index).unwrap_or(&EMPTY),
        }
    }
}

impl KlondikeTable {
    fn pile_mut(&mut self, pile_id: KlondikePileId) -> &mut model::pile::Pile {
        match pile_id {
            KlondikePileId::Stock => &mut self.stock,
            KlondikePileId::Waste => &mut self.waste,
            KlondikePileId::Foundation(suit) => match suit {
                model::card::Suit::Spades => &mut self.spades_foundation,
                model::card::Suit::Hearts => &mut self.hearts_foundation,
                model::card::Suit::Diamonds => &mut self.diamonds_foundation,
                model::card::Suit::Clubs => &mut self.clubs_foundation,
            },
            KlondikePileId::Tableaux(index) => {
                if index >= self.tableaux.len() {
                    self.tableaux.resize_with(index + 1, Default::default);
                }
                &mut self.tableaux[index]
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KlondikeTableAction {
    Deal(KlondikePileId),
    Draw(usize),
    Move(KlondikePileId, KlondikePileId, usize),
    Reveal(KlondikePileId),
}

impl<'a> model::action::Action<KlondikeTable> for KlondikeTableAction {
    fn apply_to(self, table: &mut KlondikeTable) {
        match self {
            Self::Deal(target_pile_id) => {
                let card = table.stock.take_top();
                table.pile_mut(target_pile_id).place_cards(card);
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
                    .flip_top_to(model::card::Facing::FaceUp);
            }
        }
    }
}

// impl rules::Rules<Table, Action> for Rules {
//     type Error = RulesError;
//
//     fn check_rules(&self, target: &Table, action: &Action) -> Result<(), Self::Error> {
//         match action {
//             &model::action::Deal(target_pile_id, _) => {
//                 if let PileId::Tableaux(index) = target_pile_id {
//                     snafu::ensure!(index < self.tableaux_width, PileOutOfBounds { index });
//
//                     if let Some(top_card) = target.pile(target_pile_id).top_card() {
//                         snafu::ensure!(
//                             top_card.facing == model::card::Facing::FaceDown,
//                             IllegalDealTargetFacing {
//                                 facing: top_card.facing
//                             }
//                         );
//                     }
//                 } else {
//                     return IllegalDealTarget {
//                         pile_id: target_pile_id,
//                     }
//                     .fail();
//                 }
//             }
//             &model::action::Move(source_pile_id, target_pile_id, count) => {
//                 snafu::ensure!(count > 0, EmptyMove);
//
//                 let source_top_cards = target.pile(source_pile_id).top_cards(count);
//                 snafu::ensure!(count == source_top_cards.len(), InsufficientCards);
//
//                 // We assume it's sufficient to check the facing of the bottom card of the pile.
//                 snafu::ensure!(
//                     source_top_cards[0].is_face_up(),
//                     IllegalMoveSourceFacing {
//                         facing: source_top_cards[0].facing
//                     }
//                 );
//
//                 match source_pile_id {
//                     PileId::Tableaux(index) => {
//                         snafu::ensure!(index < self.tableaux_width, PileOutOfBounds { index });
//                     }
//                     PileId::Foundation(_) => {
//                         snafu::ensure!(self.can_move_from_foundation, IllegalMoveFromFoundation);
//                         snafu::ensure!(
//                             count == 1,
//                             MayOnlyTakeSingleCard {
//                                 pile_id: source_pile_id
//                             }
//                         );
//                     }
//                     PileId::Waste => {
//                         snafu::ensure!(
//                             count == 1,
//                             MayOnlyTakeSingleCard {
//                                 pile_id: source_pile_id
//                             }
//                         );
//                     }
//                     _ => {
//                         return IllegalMoveSource {
//                             pile_id: source_pile_id,
//                         }
//                         .fail()
//                     }
//                 }
//
//                 match target_pile_id {
//                     PileId::Tableaux(index) => {
//                         snafu::ensure!(index < self.tableaux_width, PileOutOfBounds { index });
//
//                         if let Some(target_top_card) = target.pile(target_pile_id).top_card() {
//                             snafu::ensure!(
//                                 target_top_card.is_face_up(),
//                                 IllegalMoveTargetFacing {
//                                     facing: target_top_card.facing
//                                 }
//                             );
//                             snafu::ensure!(
//                                 target_top_card.rank().follows(source_top_cards[0].rank()),
//                                 TableauxMismatch {
//                                     card: source_top_cards[0].face,
//                                     mismatch: TableauxMismatchType::Follow(target_top_card.face)
//                                 }
//                             );
//                             snafu::ensure!(
//                                 target_top_card.color() != source_top_cards[0].color(),
//                                 TableauxMismatch {
//                                     card: target_top_card.face,
//                                     mismatch: TableauxMismatchType::Follow(target_top_card.face)
//                                 }
//                             );
//                         } else {
//                             snafu::ensure!(
//                                 source_top_cards[0].is_king(),
//                                 TableauxMismatch {
//                                     card: source_top_cards[0].face,
//                                     mismatch: TableauxMismatchType::Start
//                                 }
//                             );
//                         }
//                     }
//                     PileId::Foundation(suit) => {
//                         snafu::ensure!(
//                             count == 1,
//                             MayOnlyAcceptSingleCard {
//                                 pile_id: target_pile_id,
//                             }
//                         );
//
//                         // We can expect this because we checked the source had sufficient cards
//                         // above.
//                         let source_card = &source_top_cards[0];
//
//                         if let Some(target_top_card) = target.pile(target_pile_id).top_card() {
//                             snafu::ensure!(
//                                 target_top_card.color() == source_card.color(),
//                                 FoundationMismatch {
//                                     card: target_top_card.face,
//                                     mismatch: FoundationMismatchType::Follow(target_top_card.face)
//                                 }
//                             );
//                             snafu::ensure!(
//                                 target_top_card.rank().follows(source_card.rank()),
//                                 FoundationMismatch {
//                                     card: source_card.face,
//                                     mismatch: FoundationMismatchType::Follow(target_top_card.face)
//                                 }
//                             );
//                         } else {
//                             snafu::ensure!(
//                                 source_card.suit() == suit,
//                                 FoundationMismatch {
//                                     card: source_card.face,
//                                     mismatch: FoundationMismatchType::Start(suit)
//                                 }
//                             );
//                             snafu::ensure!(
//                                 source_card.is_ace(),
//                                 FoundationMismatch {
//                                     card: source_card.face,
//                                     mismatch: FoundationMismatchType::Start(suit)
//                                 }
//                             );
//                         }
//                     }
//                     _ => {
//                         return IllegalMoveTarget {
//                             pile_id: target_pile_id,
//                         }
//                         .fail();
//                     }
//                 }
//             }
//             &model::action::Reveal(target_pile_id) => {
//                 if let PileId::Tableaux(index) = target_pile_id {
//                     snafu::ensure!(index < self.tableaux_width, PileOutOfBounds { index });
//                 } else {
//                     return IllegalRevealTarget {
//                         pile_id: target_pile_id,
//                     }
//                     .fail();
//                 }
//             }
//             _ => {}
//         }
//
//         Ok(())
//     }
// }
