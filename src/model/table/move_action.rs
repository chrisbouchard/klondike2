use crate::model::{action, card, rules, table};

#[derive(Debug, Clone)]
pub struct MoveAction(super::PileId, super::PileId, usize);

impl action::Action<table::Table> for MoveAction {
    // TODO: Use Error = ! once RFC 1216 is stabilized (rust-lang/rust#35121).
    type Error = std::convert::Infallible;

    fn apply_to(self, target: &mut table::Table) -> Result<(), Self::Error> {
        let MoveAction(source_pile_id, target_pile_id, count) = self;
        let moved_cards = target.pile_mut(source_pile_id).take(count);
        target.pile_mut(target_pile_id).place(moved_cards);
        Ok(())
    }
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
pub enum MoveRulesError {
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
    IllegalSource { pile_id: table::PileId },
    #[snafu(display(""))]
    IllegalSourceFacing { facing: card::Facing },
    #[snafu(display(""))]
    IllegalTarget { pile_id: table::PileId },
    #[snafu(display(""))]
    IllegalTargetFacing { facing: card::Facing },
    #[snafu(display(""))]
    InsufficientCards,
    #[snafu(display(""))]
    PileOutOfBounds { index: usize },
    #[snafu(display(""))]
    TableauxMismatch {
        card: card::CardFace,
        mismatch: TableauxMismatchType,
    },
    #[snafu(display(""))]
    TooManyCardsForTarget { pile_id: table::PileId },
    #[snafu(display(""))]
    TooManyCardsForSource { pile_id: table::PileId },
}

impl rules::Rules<table::Table, MoveAction> for table::Rules {
    type Error = MoveRulesError;

    fn check_rules(&self, target: &table::Table, action: &MoveAction) -> Result<(), Self::Error> {
        let &MoveAction(source_pile_id, target_pile_id, count) = action;

        snafu::ensure!(count > 0, EmptyMove);

        let source_top_cards = target.pile(source_pile_id).top_cards(count);
        snafu::ensure!(count == source_top_cards.len(), InsufficientCards);

        // We assume it's sufficient to check the facing of the bottom card of the pile.
        snafu::ensure!(
            source_top_cards[0].is_face_up(),
            IllegalSourceFacing {
                facing: source_top_cards[0].facing
            }
        );

        match source_pile_id {
            table::PileId::Tableaux(index) => {
                snafu::ensure!(index < self.tableaux_width, PileOutOfBounds { index });
            }
            table::PileId::Foundation(_) => {
                snafu::ensure!(self.can_move_from_foundation, IllegalMoveFromFoundation);
                snafu::ensure!(
                    count == 1,
                    TooManyCardsForSource {
                        pile_id: source_pile_id
                    }
                );
            }
            table::PileId::Waste => {
                snafu::ensure!(
                    count == 1,
                    TooManyCardsForSource {
                        pile_id: source_pile_id
                    }
                );
            }
            _ => {
                return IllegalSource {
                    pile_id: source_pile_id,
                }
                .fail()
            }
        }

        match target_pile_id {
            table::PileId::Tableaux(index) => {
                snafu::ensure!(index < self.tableaux_width, PileOutOfBounds { index });

                if let Some(target_top_card) = target.pile(target_pile_id).top_card() {
                    snafu::ensure!(
                        target_top_card.is_face_up(),
                        IllegalTargetFacing {
                            facing: target_top_card.facing
                        }
                    );
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
            table::PileId::Foundation(suit) => {
                snafu::ensure!(
                    count == 1,
                    TooManyCardsForTarget {
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
                return IllegalTarget {
                    pile_id: target_pile_id,
                }
                .fail();
            }
        }

        Ok(())
    }
}
