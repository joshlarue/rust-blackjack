use crate::card::*;
use rand::prelude::*;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Deck {
    cards: VecDeque<Card>,
}
impl Deck {
    // new and add_card are only public because of tests in player.rs
    pub fn new() -> Deck {
        Deck {
            cards: VecDeque::<Card>::new(),
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push_front(card);
    }

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.make_contiguous().shuffle(&mut rng);
    }

    pub fn deal_card(&mut self) -> Card {
        self.cards.pop_front().unwrap()
    }
}

pub fn new_deck() -> Deck {
    let suits: Vec<Suit> = vec![Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

    let ranks: Vec<Rank> = vec![
        Rank::Ace,
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
    ];

    let mut deck = Deck::new();
    for suit in suits.iter() {
        for rank in ranks.iter() {
            deck.add_card(Card::new(suit.clone(), rank.clone()));
        }
    }

    deck.shuffle();
    deck
}
