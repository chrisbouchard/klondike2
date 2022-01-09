use enum_like::EnumLike;
use enum_like::EnumValues;

#[derive(Debug, Copy, Clone, Eq, PartialEq, derive_more::Display)]
pub enum Color {
    #[display(fmt = "Black")]
    Black,
    #[display(fmt = "Red")]
    Red,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, derive_more::Display, enum_like_derive::EnumLike)]
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
    enum_like_derive::EnumLike,
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
    pub fn of(self, suit: Suit) -> CardFace {
        CardFace { rank: self, suit }
    }

    pub fn next(self) -> Option<Self> {
        let next_discr = self.to_discr() + 1;

        if next_discr < <Self as EnumLike>::NUM_VARIANTS {
            Some(<Self as EnumLike>::from_discr(next_discr))
        } else {
            None
        }
    }

    pub fn follows(self, other: Rank) -> bool {
        other
            .next()
            .map(|other_next| self == other_next)
            .unwrap_or_default()
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
    enum_like_derive::EnumLike,
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
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, derive_more::Display, enum_like_derive::EnumLike)]
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

    pub fn face_down(self) -> Card {
        self.with_facing(Facing::FaceDown)
    }

    pub fn face_up(self) -> Card {
        self.with_facing(Facing::FaceUp)
    }

    pub fn next_in_suit(self) -> Option<Self> {
        self.rank.next().map(|rank| Self { rank, ..self })
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

    pub fn face_down(mut self) -> Self {
        self.facing = Facing::FaceDown;
        self
    }

    pub fn face_up(mut self) -> Self {
        self.facing = Facing::FaceUp;
        self
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
