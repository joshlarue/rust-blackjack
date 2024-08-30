use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, PartialEq, Clone)]
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
impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
impl Rank {
    pub fn value(self) -> u8 {
        match self {
            Rank::Ace => 10,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 10,
            Rank::Queen => 10,
            Rank::King => 10,
        }
    }
}

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}
impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Card {
        Card { suit, rank }
    }
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.suit == other.suit && self.rank == other.rank
    }
}
