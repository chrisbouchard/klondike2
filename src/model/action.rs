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
