use std::fmt;

use itertools::Itertools as _;
use rand::seq::SliceRandom as _;

use crate::model::card;

pub type IntoIter = <Vec<card::Card> as IntoIterator>::IntoIter;

pub trait Shuffle: fmt::Debug {
    fn shuffle(&mut self, cards: &mut [card::Card]);
}

#[derive(Debug, Clone)]
pub struct Deck {
    cards: Vec<card::Card>,
}

impl Deck {
    pub fn new() -> Self {
        let cards = card::Card::values_face_down().collect_vec();
        Self { cards }
    }

    pub fn new_shuffled(shuffle: &mut dyn Shuffle) -> Self {
        let mut deck = Self::new();
        deck.shuffle(shuffle);
        deck
    }

    pub fn shuffle(&mut self, shuffle: &mut dyn Shuffle) {
        shuffle.shuffle(&mut self.cards);
    }
}

impl IntoIterator for Deck {
    type Item = card::Card;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.into_iter()
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NoShuffle;

impl Shuffle for NoShuffle {
    fn shuffle(&mut self, _cards: &mut [card::Card]) {}
}

#[derive(Debug, Clone, Copy)]
pub struct RandomShuffle;

impl Shuffle for RandomShuffle {
    fn shuffle(&mut self, cards: &mut [card::Card]) {
        cards.shuffle(&mut rand::thread_rng());
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UnShuffle;

impl Shuffle for UnShuffle {
    fn shuffle(&mut self, cards: &mut [card::Card]) {
        cards.sort();
    }
}
