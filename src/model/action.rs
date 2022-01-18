use std::{error, fmt};

pub trait Action<T>: fmt::Debug + Clone + 'static {
    type Error: error::Error + 'static;

    fn apply_to(self, target: &mut T) -> Result<(), Self::Error>;
}

pub trait Actionable<A>: Sized {
    type Error: error::Error + 'static;

    fn apply(&mut self, action: A) -> Result<(), Self::Error>;

    fn apply_all<I>(&mut self, actions: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = A>,
    {
        actions
            .into_iter()
            .map(|action| self.apply(action))
            .collect()
    }
}

impl<A, T> Actionable<A> for T
where
    A: Action<Self>,
{
    type Error = A::Error;

    fn apply(&mut self, action: A) -> Result<(), Self::Error> {
        action.apply_to(self)
    }
}
