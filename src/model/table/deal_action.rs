use crate::model::{action, card, rules, table};

#[derive(Debug, Clone)]
pub struct DealAction(table::PileId, card::Card);

impl action::Action<table::Table> for DealAction {
    // TODO: Use Error = ! once RFC 1216 is stabilized (rust-lang/rust#35121).
    type Error = std::convert::Infallible;

    fn apply_to(self, target: &mut table::Table) -> Result<(), Self::Error> {
        let DealAction(target_pile_id, card) = self;
        target.pile_mut(target_pile_id).place_one(card);
        Ok(())
    }
}

#[derive(Debug, snafu::Snafu)]
pub enum DealRulesError {
    #[snafu(display(""))]
    IllegalTarget { pile_id: table::PileId },
    #[snafu(display(""))]
    IllegalTargetFacing { facing: card::Facing },
    #[snafu(display(""))]
    PileOutOfBounds { index: usize },
}

impl rules::Rules<table::Table, DealAction> for table::Rules {
    type Error = DealRulesError;

    fn check_rules(&self, target: &table::Table, action: &DealAction) -> Result<(), Self::Error> {
        let &DealAction(target_pile_id, _) = action;

        if let table::PileId::Tableaux(index) = target_pile_id {
            snafu::ensure!(index < self.tableaux_width, PileOutOfBounds { index });

            if let Some(top_card) = target.pile(target_pile_id).top_card() {
                snafu::ensure!(
                    top_card.facing == card::Facing::FaceDown,
                    IllegalTargetFacing {
                        facing: top_card.facing
                    }
                );
            }
        } else {
            return IllegalTarget {
                pile_id: target_pile_id,
            }
            .fail();
        }

        Ok(())
    }
}
