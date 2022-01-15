use std::fmt;

pub trait Dealer<A>: Clone + fmt::Debug {
    type Context<'a>;
    type Iter: Iterator<Item = A> + Clone;

    fn deal(&self, context: Self::Context<'_>) -> Self::Iter;
}
