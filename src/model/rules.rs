use std::fmt;

pub trait Rules<A>: Clone + fmt::Debug {
    type Context<'a>;
    type Error: 'static;

    fn is_valid(&self, action: &A, context: Self::Context<'_>) -> Result<(), Self::Error>;
}
