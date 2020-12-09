use std::iter;
use std::mem;
use std::slice;
use std::vec;

use enum_iterator::IntoEnumIterator;
use itertools::Itertools;

use super::card;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum PileId {
    Stock,
    Waste,
    Foundation(card::Suit),
    Tableaux(usize),
}

impl PileId {
    pub fn standard_iter() -> impl Iterator<Item = PileId> {
        velcro::iter![
            PileId::Stock,
            PileId::Waste,
            ..card::Suit::into_enum_iter().map(PileId::Foundation)
        ]
    }

    pub fn full_iter(tableaux_width: usize) -> impl Iterator<Item = PileId> {
        velcro::iter![
            ..Self::standard_iter(),
            ..(0..tableaux_width).map(PileId::Tableaux)
        ]
    }
}

pub type Iter<'a> = slice::Iter<'a, card::Card>;
pub type IntoIter = vec::IntoIter<card::Card>;

#[derive(Debug, Clone, Default)]
pub struct Pile {
    cards: Vec<card::Card>,
}

impl Pile {
    pub const fn new() -> Self {
        Pile { cards: Vec::new() }
    }

    pub fn iter(&self) -> Iter {
        self.cards.iter()
    }

    pub fn top_card(&self) -> Option<&card::Card> {
        self.cards.first()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn flip(&mut self) {
        let cards = mem::take(&mut self.cards)
            .into_iter()
            .rev()
            .map(card::Card::reversed)
            .collect_vec();
        self.cards = cards;
    }

    pub fn flip_top(&mut self) {
        self.cards.first_mut().map(card::Card::reverse);
    }

    pub fn flip_top_to(&mut self, facing: card::Facing) {
        if let Some(top_card) = self.cards.first_mut() {
            top_card.facing = facing;
        }
    }

    pub fn flipped(mut self) -> Self {
        self.flip();
        self
    }

    pub fn place(&mut self, other: Self) {
        self.cards.extend(other.cards)
    }

    pub fn place_one(&mut self, card: card::Card) {
        self.cards.push(card)
    }

    pub fn place_cards<I>(&mut self, cards: I)
    where
        I: IntoIterator<Item = card::Card>,
    {
        self.cards.extend(cards)
    }

    pub fn take(&mut self, count: usize) -> Self {
        let start_index = self.len().saturating_sub(count);
        self.cards.drain(start_index..).collect::<Self>()
    }

    pub fn take_top(&mut self) -> Self {
        let cards = self.cards.pop().into_iter().collect_vec();
        Pile { cards }
    }

    pub fn take_all(&mut self) -> Self {
        let cards = mem::take(&mut self.cards);
        Pile { cards }
    }
}

impl IntoIterator for Pile {
    type Item = card::Card;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.into_iter()
    }
}

impl<'a> IntoIterator for &'a Pile {
    type Item = &'a card::Card;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl iter::FromIterator<card::Card> for Pile {
    fn from_iter<T: IntoIterator<Item = card::Card>>(iter: T) -> Self {
        Pile {
            cards: velcro::vec![..iter],
        }
    }
}
