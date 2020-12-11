use std::vec;

use enum_iterator::IntoEnumIterator;
use itertools::Itertools;
use rand::seq::SliceRandom;

use super::card;

pub type IntoIter = vec::IntoIter<card::Card>;

#[derive(Debug, Copy, Clone)]
pub enum Shuffle {
    None,
    Random,
}

impl Shuffle {
    pub fn shuffle(self, cards: &mut [card::Card]) {
        match self {
            Self::None => {}
            Self::Random => {
                cards.shuffle(&mut rand::thread_rng());
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Deck {
    cards: Vec<card::Card>,
}

impl Deck {
    pub fn new() -> Self {
        let cards = card::Suit::into_enum_iter()
            .cartesian_product(card::Rank::into_enum_iter())
            .map(|(suit, rank)| card::Card {
                suit,
                rank,
                facing: card::Facing::FaceDown,
            })
            .collect_vec();
        Self { cards }
    }

    pub fn shuffle(&mut self, shuffler: Shuffle) {
        shuffler.shuffle(&mut self.cards);
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
