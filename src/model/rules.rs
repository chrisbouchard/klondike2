use std::{error, fmt};

use snafu::ResultExt as _;

use crate::model::action;
use crate::model::action::Actionable as _;

pub trait Rules<A>: fmt::Debug + Clone {
    type Context<'a>;
    type Error: fmt::Debug + error::Error + 'static;

    fn validate(&self, action: &A, context: &Self::Context<'_>) -> Result<(), Self::Error>;
}

#[derive(Debug, Clone)]
pub struct RulesGuard<R, T> {
    rules: R,
    target: T,
}

#[derive(Debug, snafu::Snafu)]
pub enum RulesGuardError<RE, AE, A>
where
    RE: error::Error + 'static,
    AE: error::Error + 'static,
{
    RuleError { action: A, source: RE },
    ActionError { action: A, source: AE },
}

impl<R, T> RulesGuard<R, T> {
    pub fn new(rules: R, target: T) -> Self {
        Self { rules, target }
    }

    pub fn apply_guarded<A>(
        &mut self,
        action: A,
        context: &R::Context<'_>,
    ) -> Result<(), RulesGuardError<R::Error, A::Error, A>>
    where
        A: action::Action<T>,
        R: Rules<A>,
    {
        self.rules.validate(&action, context).context(RuleError {
            action: action.clone(),
        })?;
        self.target
            .apply(action.clone())
            .context(ActionError { action })
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
            .try_for_each(|rules| rules.validate(action, context))
    }
}
