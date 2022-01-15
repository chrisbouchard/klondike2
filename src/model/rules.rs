use std::{convert, fmt};

use super::action;

pub trait Rules<A>: fmt::Debug + Clone {
    type Context<'a>;
    type Error: fmt::Debug + 'static;

    fn validate(&self, action: &A, context: Self::Context<'_>) -> Result<(), Self::Error>;

    fn guard<T>(self, target: T) -> RulesGuard<Self, T> {
        RulesGuard {
            rules: self,
            target,
        }
    }
}

pub struct RulesGuard<R, T> {
    rules: R,
    target: T,
}

impl<R, T> RulesGuard<R, T> {
    pub fn apply_guarded<A>(&mut self, action: A, context: R::Context<'_>) -> Result<(), R::Error>
    where
        A: action::Action<T>,
        R: Rules<A>,
    {
        self.rules.validate(&action, context)?;
        action.apply_to(&mut self.target);
        Ok(())
    }
}

impl<R, T> convert::AsRef<T> for RulesGuard<R, T> {
    fn as_ref(&self) -> &T {
        &self.target
    }
}
