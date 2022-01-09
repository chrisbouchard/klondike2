use super::action::Action;

pub trait Rules<A, T>
where
    A: Action<T>,
{
    type State<'a>;

    fn valid_actions<'a, 'b>(&'a self, state: Self::State<'b>) -> Vec<A>;

    fn is_valid_action<'a, 'b>(&'a self, state: Self::State<'b>, action: A) -> bool
    where
        A: Eq,
    {
        self.valid_actions(state).into_iter().any(|a| a.eq(&action))
    }
}
