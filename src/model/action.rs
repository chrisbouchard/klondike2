pub trait Action<T> {
    fn apply_to(self, target: &mut T);
}

pub trait Actionable<A> {
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
    A: Action<T>,
{
    fn apply(&mut self, action: A) {
        action.apply_to(self);
    }
}

impl<A, T> Actionable<A> for dyn AsMut<T>
where
    T: Actionable<A>,
{
    fn apply(&mut self, action: A) {
        self.as_mut().apply(action);
    }
}
