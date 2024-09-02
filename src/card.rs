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
        match self {
            Suit::Clubs => write!(f, "Clubs"),
            Suit::Diamonds => write!(f, "Diamonds"),
            Suit::Hearts => write!(f, "Hearts"),
            Suit::Spades => write!(f, "Spades"),
        }
    }
}

impl Suit {
    pub fn unicode_suit(self) -> char {
        match self {
            Suit::Clubs => 'D',
            Suit::Diamonds => 'C',
            Suit::Hearts => 'B',
            Suit::Spades => 'A',
        }
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
        match self {
            Rank::Ace => write!(f, "Ace"),
            Rank::Two => write!(f, "Two"),
            Rank::Three => write!(f, "Three"),
            Rank::Four => write!(f, "Four"),
            Rank::Five => write!(f, "Five"),
            Rank::Six => write!(f, "Six"),
            Rank::Seven => write!(f, "Seven"),
            Rank::Eight => write!(f, "Eight"),
            Rank::Nine => write!(f, "Nine"),
            Rank::Ten => write!(f, "Ten"),
            Rank::Jack => write!(f, "Jack"),
            Rank::Queen => write!(f, "Queen"),
            Rank::King => write!(f, "King"),
        }
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

    pub fn unicode_rank(self) -> u8 {
        match self {
            Rank::Ace => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 0xA,
            Rank::Jack => 0xB,
            Rank::Queen => 0xD,
            Rank::King => 0xE,
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
        let mut card_unicode = String::new();
        card_unicode.push_str("1F0");

        let suit_unicode = self.suit.clone().unicode_suit();
        let rank_unicode = &self.rank.clone().unicode_rank();

        card_unicode.push(suit_unicode);
        card_unicode.push_str(&format!("{:X}", rank_unicode));

        let card_unicode_u32: u32 = u32::from_str_radix(card_unicode.as_str(), 16)
            .expect("Unable to parse card's unicode into a u32.");
        let card_char = std::char::from_u32(card_unicode_u32).unwrap();

        write!(f, " {} ", card_char)
    }
}
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.suit == other.suit && self.rank == other.rank
    }
}
