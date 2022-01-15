use std::fmt;

use super::action::Actionable as _;

use super::action;

pub trait Rules<A>: fmt::Debug + Clone {
    type Context<'a>;
    type Error: fmt::Debug + 'static;

    fn validate(&self, action: &A, context: &Self::Context<'_>) -> Result<(), Self::Error>;
}

#[derive(Debug, Clone)]
pub struct RulesGuard<R, T> {
    rules: R,
    target: T,
}

impl<R, T> RulesGuard<R, T> {
    pub fn new(rules: R, target: T) -> Self {
        Self { rules, target }
    }

    pub fn apply_guarded<A>(&mut self, action: A, context: &R::Context<'_>) -> Result<(), R::Error>
    where
        A: action::Action<T>,
        R: Rules<A>,
    {
        self.rules.validate(&action, context)?;
        self.target.apply(action);
        Ok(())
    }

    pub fn rules(&self) -> &R {
        &self.rules
    }

    pub fn target(&self) -> &T {
        &self.target
    }

    pub fn set_target(&mut self, target: T) {
        self.target = target;
    }
}

#[derive(Debug, Clone)]
pub struct CompoundRules<R> {
    rules: Vec<R>,
}

impl<A, R> Rules<A> for CompoundRules<R>
where
    R: Rules<A>,
{
    type Context<'a> = R::Context<'a>;
    type Error = R::Error;

    fn validate(&self, action: &A, context: &Self::Context<'_>) -> Result<(), Self::Error> {
        self.rules
            .iter()
            .map(|rules| rules.validate(action, context))
            .collect()
    }
}
