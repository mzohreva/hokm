
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Suit {
    Hearts,
    Clubs,
    Diamonds,
    Spades,
}

impl Suit {
    pub fn from_u8(x: u8) -> Suit {
        assert!(x < 4, "suit must be < 4");
        unsafe { ::std::mem::transmute(x) }
    }
    pub fn all_suits() -> &'static [Suit] {
        &[Suit::Hearts, Suit::Clubs, Suit::Diamonds, Suit::Spades]
    }
}

impl fmt::Debug for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Suit::Hearts   => write!(f, "♥"),
            Suit::Clubs    => write!(f, "♣"),
            Suit::Diamonds => write!(f, "♦"),
            Suit::Spades   => write!(f, "♠"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Rank {
    Two = 2,
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
    Ace,
}

impl fmt::Debug for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rank::Two   => write!(f, "2"),
            Rank::Three => write!(f, "3"),
            Rank::Four  => write!(f, "4"),
            Rank::Five  => write!(f, "5"),
            Rank::Six   => write!(f, "6"),
            Rank::Seven => write!(f, "7"),
            Rank::Eight => write!(f, "8"),
            Rank::Nine  => write!(f, "9"),
            Rank::Ten   => write!(f, "X"),
            Rank::Jack  => write!(f, "J"),
            Rank::Queen => write!(f, "Q"),
            Rank::King  => write!(f, "K"),
            Rank::Ace   => write!(f, "A"),
        }
    }
}

impl Rank {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
    pub fn from_u8(x: u8) -> Rank {
        assert!(x >= 2 && x < 15, "rank must be >= 2 and < 15");
        unsafe { ::std::mem::transmute(x) }
    }
    pub fn all_ranks() -> &'static [Rank] {
        &[
            Rank::Two, Rank::Three, Rank::Four,
            Rank::Five, Rank::Six, Rank::Seven,
            Rank::Eight, Rank::Nine, Rank::Ten,
            Rank::Jack, Rank::Queen, Rank::King,
            Rank::Ace,
        ]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Card(u8);

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}{:?}", self.rank(), self.suit())
    }
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card(suit as u8 * 13 + rank as u8 - 2)
    }
    pub fn rank(&self) -> Rank {
        Rank::from_u8(self.0 % 13 + 2)
    }
    pub fn suit(&self) -> Suit {
        Suit::from_u8(self.0 / 13)
    }
}
