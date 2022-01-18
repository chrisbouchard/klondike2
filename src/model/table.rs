use std::fmt::Debug;

use crate::model::{card, pile};

pub trait Table: Debug + Clone {
    type PileId: Debug + Copy + Eq;

    fn new_with_cards<I>(cards: I) -> Self
    where
        I: IntoIterator<Item = card::Card>;

    fn pile(&self, pile_id: Self::PileId) -> &pile::Pile;
}
