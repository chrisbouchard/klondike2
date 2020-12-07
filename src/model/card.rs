use enum_iterator::IntoEnumIterator;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Color {
    Black,
    Red,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Facing {
    FaceDown,
    FaceUp,
}

impl Facing {
    pub fn reversed(self) -> Self {
        match self {
            Self::FaceDown => Self::FaceUp,
            Self::FaceUp => Self::FaceDown,
        }
    }
}

/// Enum representing the valid ranks for a card. The values for cards are a fixed set, so we use
/// an enum rather than a dumb wrapper around `u8`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, IntoEnumIterator)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, IntoEnumIterator)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Suit {
    pub fn color(self) -> Color {
        match self {
            Self::Spades | Self::Clubs => Color::Black,
            Self::Hearts | Self::Diamonds => Color::Red,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
    pub facing: Facing,
}

impl Card {
    pub fn color(&self) -> Color {
        self.suit.color()
    }

    pub fn is_ace(&self) -> bool {
        self.rank == Rank::Ace
    }

    pub fn is_king(&self) -> bool {
        self.rank == Rank::King
    }

    pub fn is_face_down(&self) -> bool {
        self.facing == Facing::FaceDown
    }

    pub fn is_face_up(&self) -> bool {
        self.facing == Facing::FaceUp
    }

    pub fn reverse(&mut self) {
        self.facing = self.facing.reversed()
    }

    pub fn reversed(mut self) -> Self {
        self.reverse();
        self
    }
}
