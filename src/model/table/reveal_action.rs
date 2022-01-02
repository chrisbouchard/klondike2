use crate::model::{action, card, rules, table};

#[derive(Debug, Clone)]
pub struct RevealAction(super::PileId);

impl action::Action<table::Table> for RevealAction {
    // TODO: Use Error = ! once RFC 1216 is stabilized (rust-lang/rust#35121).
    type Error = std::convert::Infallible;

    fn apply_to(self, target: &mut table::Table) -> Result<(), Self::Error> {
        let RevealAction(target_pile_id) = self;
        target
            .pile_mut(target_pile_id)
            .flip_top_to(card::Facing::FaceUp);
        Ok(())
    }
}

#[derive(Debug, snafu::Snafu)]
pub enum RevealRulesError {
    #[snafu(display(""))]
    IllegalTarget { pile_id: table::PileId },
    #[snafu(display(""))]
    PileOutOfBounds { index: usize },
}

impl rules::Rules<table::Table, RevealAction> for table::Rules {
    type Error = RevealRulesError;

    fn check_rules(
        &self,
        _target: &table::Table,
        action: &RevealAction,
    ) -> Result<(), Self::Error> {
        let &RevealAction(target_pile_id) = action;

        if let table::PileId::Tableaux(index) = target_pile_id {
            snafu::ensure!(index < self.tableaux_width, PileOutOfBounds { index });
        } else {
            return IllegalTarget {
                pile_id: target_pile_id,
            }
            .fail();
        }

        Ok(())
    }
}
