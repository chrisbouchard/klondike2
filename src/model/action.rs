use std::error;
use std::fmt;
use std::rc::Rc;
use std::sync::Arc;

pub trait Action<T>: Clone + fmt::Debug {
    type Error: error::Error + 'static;

    fn apply_to(self, target: &mut T) -> Result<(), Self::Error>;
}

impl<A, T> Action<Rc<T>> for A
where
    A: Action<T>,
    T: Clone,
{
    type Error = A::Error;

    fn apply_to(self, target: &mut Rc<T>) -> Result<(), Self::Error> {
        let inner_target = Rc::make_mut(target);
        self.apply_to(inner_target)
    }
}

impl<A, T> Action<Arc<T>> for A
where
    A: Action<T>,
    T: Clone,
{
    type Error = A::Error;

    fn apply_to(self, target: &mut Arc<T>) -> Result<(), Self::Error> {
        let inner_target = Arc::make_mut(target);
        self.apply_to(inner_target)
    }
}

pub trait Actionable<A> {
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
