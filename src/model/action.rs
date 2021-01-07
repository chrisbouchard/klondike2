use std::rc::Rc;
use std::sync::Arc;

pub type Result<E> = std::result::Result<(), E>;

pub trait Action<T, E> {
    fn apply_to(self, target: &mut T) -> Result<E>;
}

impl<A, T, E> Action<Rc<T>, E> for A
where
    A: Action<T, E>,
    T: Clone,
{
    fn apply_to(self, target: &mut Rc<T>) -> Result<E> {
        let inner_target = Rc::make_mut(target);
        self.apply_to(inner_target)
    }
}

impl<A, T, E> Action<Arc<T>, E> for A
where
    A: Action<T, E>,
    T: Clone,
{
    fn apply_to(self, target: &mut Arc<T>) -> Result<E> {
        let inner_target = Arc::make_mut(target);
        self.apply_to(inner_target)
    }
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
