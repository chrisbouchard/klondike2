use crate::model::{action, rules, table};

#[derive(Debug, Clone)]
pub struct DrawAction(usize);

impl action::Action<table::Table> for DrawAction {
    // TODO: Use Error = ! once RFC 1216 is stabilized (rust-lang/rust#35121).
    type Error = std::convert::Infallible;

    fn apply_to(self, target: &mut table::Table) -> Result<(), Self::Error> {
        let DrawAction(count) = self;

        if target.stock.is_empty() {
            let replacement_cards = target.waste.take_all().flipped();
            target.stock.place(replacement_cards);
        } else {
            let drawn_cards = target.stock.take(count).flipped();
            target.waste.place(drawn_cards);
        }

        Ok(())
    }
}

impl rules::Rules<table::Table, DrawAction> for table::Rules {
    // TODO: Use Error = ! once RFC 1216 is stabilized (rust-lang/rust#35121).
    type Error = std::convert::Infallible;

    fn check_rules(&self, _target: &table::Table, _action: &DrawAction) -> Result<(), Self::Error> {
        Ok(())
    }
}
