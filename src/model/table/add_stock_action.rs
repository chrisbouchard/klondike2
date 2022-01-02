use crate::model::{action, pile, rules, table};

#[derive(Debug, Clone)]
pub struct AddStockAction(pile::Pile);

impl action::Action<table::Table> for AddStockAction {
    // TODO: Use Error = ! once RFC 1216 is stabilized (rust-lang/rust#35121).
    type Error = std::convert::Infallible;

    fn apply_to(self, target: &mut table::Table) -> Result<(), Self::Error> {
        let AddStockAction(pile) = self;
        target.stock.place(pile);
        Ok(())
    }
}

impl rules::Rules<table::Table, AddStockAction> for table::Rules {
    // TODO: Use Error = ! once RFC 1216 is stabilized (rust-lang/rust#35121).
    type Error = std::convert::Infallible;

    fn check_rules(
        &self,
        _target: &table::Table,
        _action: &AddStockAction,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
