use std::convert;

use itertools::Itertools as _;

#[derive(Debug, Copy, Clone, Eq, PartialEq, derive_more::Display)]
pub enum Color {
    #[display(fmt = "Black")]
    Black,
    #[display(fmt = "Red")]
    Red,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, derive_more::Display)]
pub enum Facing {
    #[display(fmt = "Face down")]
    FaceDown,
    #[display(fmt = "Face up")]
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
#[derive(
    Debug,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    derive_more::Display,
    num_enum::IntoPrimitive,
    num_enum::TryFromPrimitive,
)]
#[repr(u8)]
pub enum Rank {
    #[display(fmt = "Ace")]
    Ace,
    #[display(fmt = "Two")]
    Two,
    #[display(fmt = "Three")]
    Three,
    #[display(fmt = "Four")]
    Four,
    #[display(fmt = "Five")]
    Five,
    #[display(fmt = "Six")]
    Six,
    #[display(fmt = "Seven")]
    Seven,
    #[display(fmt = "Eight")]
    Eight,
    #[display(fmt = "Nine")]
    Nine,
    #[display(fmt = "Ten")]
    Ten,
    #[display(fmt = "Jack")]
    Jack,
    #[display(fmt = "Queen")]
    Queen,
    #[display(fmt = "King")]
    King,
}

impl Rank {
    pub fn values() -> impl Iterator<Item = Self> + ExactSizeIterator + Clone {
        (u8::from(Self::Ace)..=u8::from(Self::King))
            .map(convert::TryFrom::try_from)
            .map(Result::unwrap)
    }
}

#[derive(
    Debug,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    derive_more::Display,
    num_enum::IntoPrimitive,
    num_enum::TryFromPrimitive,
)]
#[repr(u8)]
pub enum Suit {
    #[display(fmt = "Spades")]
    Spades,
    #[display(fmt = "Hearts")]
    Hearts,
    #[display(fmt = "Diamonds")]
    Diamonds,
    #[display(fmt = "Clubs")]
    Clubs,
}

impl Suit {
    pub fn color(self) -> Color {
        match self {
            Self::Spades | Self::Clubs => Color::Black,
            Self::Hearts | Self::Diamonds => Color::Red,
        }
    }

    pub fn values() -> impl Iterator<Item = Self> + ExactSizeIterator + Clone {
        (u8::from(Self::Spades)..=u8::from(Self::Clubs))
            .map(convert::TryFrom::try_from)
            .map(Result::unwrap)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, derive_more::Display)]
#[display(fmt = "{} of {}", rank, suit)]
pub struct CardFace {
    pub suit: Suit,
    pub rank: Rank,
}

impl CardFace {
    pub fn color(&self) -> Color {
        self.suit.color()
    }

    pub fn is_ace(&self) -> bool {
        self.rank == Rank::Ace
    }

    pub fn is_king(&self) -> bool {
        self.rank == Rank::King
    }

    pub fn with_facing(self, facing: Facing) -> Card {
        Card { face: self, facing }
    }

    pub fn values() -> impl Iterator<Item = Self> {
        Suit::values()
            .cartesian_product(Rank::values())
            .map(|(suit, rank)| CardFace { suit, rank })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Card {
    pub face: CardFace,
    pub facing: Facing,
}

impl Card {
    pub fn suit(&self) -> Suit {
        self.face.suit
    }

    pub fn rank(&self) -> Rank {
        self.face.rank
    }

    pub fn color(&self) -> Color {
        self.face.color()
    }

    pub fn is_ace(&self) -> bool {
        self.face.is_ace()
    }

    pub fn is_king(&self) -> bool {
        self.face.is_king()
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

    pub fn values_with_facing(facing: Facing) -> impl Iterator<Item = Self> {
        CardFace::values().map(move |face| Card { face, facing })
    }

    pub fn values_face_down() -> impl Iterator<Item = Self> {
        Self::values_with_facing(Facing::FaceDown)
    }

    pub fn values_face_up() -> impl Iterator<Item = Self> {
        Self::values_with_facing(Facing::FaceUp)
    }
}
