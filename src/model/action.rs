pub trait Action<T>: Clone {
    fn apply_to(self, target: T) -> T;
}

pub trait Actionable<A>: Sized {
    fn apply(self, action: A) -> Self;

    fn apply_all<I>(mut self, actions: I) -> Self
    where
        I: IntoIterator<Item = A>,
    {
        for action in actions {
            self = self.apply(action);
        }

        self
    }
}

impl<A, T> Actionable<A> for T
where
    A: Action<Self>,
{
    fn apply(self, action: A) -> Self {
        action.apply_to(self)
    }
}
