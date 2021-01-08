use std::error;
use std::fmt;

use snafu::ResultExt as _;

use super::action;

pub trait Rules<T, A> {
    type Error: error::Error + Sized + 'static;

    fn check_rules(&self, target: &T, action: &A) -> Result<(), Self::Error>;
}

#[derive(Debug, snafu::Snafu)]
pub enum Error<E, U, A>
where
    E: error::Error + 'static,
    U: error::Error + 'static,
    A: fmt::Debug + 'static,
{
    Invalid { source: E, action: A },
    Underlying { source: U, action: A },
}

#[derive(Debug, Clone)]
pub struct RulesGuard<R, T> {
    rules: R,
    inner: T,
}

impl<R, T> Default for RulesGuard<R, T>
where
    R: Default,
    T: Default,
{
    fn default() -> Self {
        RulesGuard {
            rules: Default::default(),
            inner: Default::default(),
        }
    }
}

impl<R, T, A> action::Action<RulesGuard<R, T>> for A
where
    R: Rules<T, A>,
    T: action::Actionable<A>,
    A: action::Action<T> + 'static,
{
    type Error = Error<R::Error, T::Error, A>;

    fn apply_to(self, target: &mut RulesGuard<R, T>) -> Result<(), Self::Error> {
        target
            .rules
            .check_rules(&target.inner, &self)
            .context(Invalid { action: self })
            .and_then(|()| {
                target
                    .inner
                    .apply(self)
                    .context(Underlying { action: self })
            })
    }
}
