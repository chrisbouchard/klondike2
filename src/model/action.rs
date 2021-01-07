pub type Result<E> = std::result::Result<(), E>;

pub trait Action<T, E> {
    fn apply_to(self, target: &mut T) -> Result<E>;
}

pub trait Actionable<A, E> {
    fn apply(&mut self, action: A) -> Result<E>;

    fn apply_all<I>(&mut self, actions: I) -> Result<E>
    where
        I: IntoIterator<Item = A>,
    {
        actions
            .into_iter()
            .map(|action| self.apply(action))
            .collect()
    }
}

impl<A, T, E> Actionable<A, E> for T
where
    A: Action<T, E>,
{
    fn apply(&mut self, action: A) -> Result<E> {
        action.apply_to(self)
    }
}

impl<A, T, E> Actionable<A, E> for dyn AsMut<T>
where
    T: Actionable<A, E>,
{
    fn apply(&mut self, action: A) -> Result<E> {
        self.as_mut().apply(action)
    }
}
