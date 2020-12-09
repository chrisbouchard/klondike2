use derivative::Derivative;
use enum_iterator::IntoEnumIterator;
use itertools::Itertools;
use rand::seq::SliceRandom;

use super::card;

pub trait Shuffler {
    fn shuffle(&mut self, cards: &mut [card::Card]);
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

    pub fn shuffle<S>(&mut self, shuffler: &mut S)
    where
        S: Shuffler,
    {
        shuffler.shuffle(&mut self.cards);
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct NoOpShuffler;

impl Shuffler for NoOpShuffler {
    fn shuffle(&mut self, _: &mut [card::Card]) {}
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct RandomShuffler<R>
where
    R: ?Sized,
{
    #[derivative(Debug = "ignore")]
    pub rng: R,
}

impl<R> Shuffler for RandomShuffler<R>
where
    R: rand::Rng + ?Sized,
{
    fn shuffle(&mut self, cards: &mut [card::Card]) {
        cards.shuffle(&mut self.rng);
    }
}
