use std::{iter, mem};

use itertools::Itertools as _;

use super::card;

pub type Iter<'a> = <&'a [card::Card] as IntoIterator>::IntoIter;
pub type IntoIter = <Vec<card::Card> as IntoIterator>::IntoIter;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Pile {
    cards: Vec<card::Card>,
}

impl Pile {
    pub const fn new() -> Self {
        Self { cards: Vec::new() }
    }

    pub fn new_with_cards<I>(cards: I) -> Self
    where
        I: IntoIterator<Item = card::Card>,
    {
        let mut pile = Self::new();
        pile.place_cards(cards);
        pile
    }

    pub fn iter(&self) -> Iter {
        self.cards.iter()
    }

    pub fn top_card(&self) -> Option<&card::Card> {
        self.cards.first()
    }

    pub fn top_cards(&self, count: usize) -> &[card::Card] {
        let start_index = self.len().saturating_sub(count);
        &self.cards[start_index..]
    }

    /// Get the slice of face-up cards on top of the pile. If there are no face-up cards on top
    /// this method will return an empty slice. If there are no face-down cards, this method will
    /// return a slice of the whole pile.
    pub fn top_face_up_cards(&self) -> &[card::Card] {
        // Find the index of the last face-up card immediately after a face-down card, after which
        // all cards will be face-up. We search from the end because the pile is stored bottom to
        // top in the vector.
        let last_face_up_card_index = self
            .iter()
            .enumerate()
            // Look for the index of the first face-down card from the end, and if we find it we
            // add one to get the index of the face-up card after it. This may put us one off the
            // end of the vector (if there are no face-up cards), but that's ok; we'll just return
            // an empty slice later.
            .rfind(|(_, card)| card.is_face_down())
            .map(|(index, _)| index + 1)
            // If there are no face-down cards, use index 0 so we get the whole pile.
            .unwrap_or(0);

        &self.cards[last_face_up_card_index..]
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn is_face_down(&self) -> bool {
        self.cards.iter().all(card::Card::is_face_down)
    }

    pub fn is_face_up(&self) -> bool {
        self.cards.iter().all(card::Card::is_face_up)
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

impl Default for Pile {
    fn default() -> Self {
        Pile::new()
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
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = card::Card>,
    {
        Self::new_with_cards(iter)
    }
}
