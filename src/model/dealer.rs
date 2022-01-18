use std::fmt;

pub trait Dealer: fmt::Debug + Clone {
    type Action;
    type Context<'a>;
    type Iter: Iterator<Item = Self::Action> + Clone;

    fn deal(&self, context: Self::Context<'_>) -> Self::Iter;
}
