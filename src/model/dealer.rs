use std::fmt::Debug;

pub trait Dealer: Debug + Clone {
    type Action;
    type Context<'a>;
    type Iter: Iterator<Item = Self::Action> + Clone;

    fn deal(&self, context: Self::Context<'_>) -> Self::Iter;
}
