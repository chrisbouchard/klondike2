use std::fmt;

pub trait Dealer: Clone + fmt::Debug {
    type Action;
    type Context<'a>;
    type Iter: Iterator<Item = Self::Action> + Clone;

    fn deal(&self, context: Self::Context<'_>) -> Self::Iter;
}
