use std::fmt;

pub trait Action<T>: fmt::Debug + Clone {
    fn apply_to(self, target: &mut T);
}

pub trait Actionable<A>: Sized {
    fn apply(&mut self, action: A);

    fn apply_all<I>(&mut self, actions: I)
    where
        I: IntoIterator<Item = A>,
    {
        for action in actions {
            self.apply(action);
        }
    }
}

impl<A, T> Actionable<A> for T
where
    A: Action<Self>,
{
    fn apply(&mut self, action: A) {
        action.apply_to(self);
    }
}
