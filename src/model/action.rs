pub trait Actionable {
    type Action;

    fn apply(&mut self, action: Self::Action);

    fn apply_all<I>(&mut self, actions: I)
    where
        I: IntoIterator<Item = Self::Action>,
    {
        for action in actions {
            self.apply(action);
        }
    }
}

impl<T> Actionable for dyn AsMut<T>
where
    T: Actionable,
{
    type Action = T::Action;

    fn apply(&mut self, action: Self::Action) {
        self.as_mut().apply(action);
    }

    fn apply_all<I>(&mut self, actions: I)
    where
        I: IntoIterator<Item = Self::Action>,
    {
        let target = self.as_mut();

        for action in actions {
            target.apply(action);
        }
    }
}
