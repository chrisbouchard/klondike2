use std::fmt;

use crate::model::{card, pile};

pub trait Table: fmt::Debug + Clone {
    type PileId: fmt::Debug + Copy + Eq;

    fn new_with_cards<I>(cards: I) -> Self
    where
        I: IntoIterator<Item = card::Card>;

    fn pile(&self, pile_id: Self::PileId) -> &pile::Pile;
}
