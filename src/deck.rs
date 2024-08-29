pub mod deck {
    use std::collections::VecDeque;
    use std::fmt;

    enum Suits {
        Clubs,
        Diamonds,
        Hearts,
        Spades,
    }

    enum Ranks {
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

    #[derive(Debug)]
    struct Card {
        suit: String,
        rank: String,
    }
    impl Card {
        fn new(suit: &str, rank: &str) -> Card {
            Card {
                suit: suit.to_string(),
                rank: rank.to_string(),
            }
        }
    }
    impl fmt::Display for Card {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} of {}", self.rank, self.suit)
        }
    }

    #[derive(Debug)]
    pub struct Deck {
        cards: VecDeque<Card>,
    }
    impl Deck {
        fn new() -> Deck {
            Deck {
                cards: VecDeque::<Card>::new(),
            }
        }

        fn add_card(&mut self, card: Card) {
            self.cards.push_front(card);
        }
    }

    pub fn new_deck() -> Deck {
        let suits: Vec<&str> = vec!["Clubs", "Diamonds", "Hearts", "Spades"];

        let ranks: Vec<&str> = vec![
            "Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack",
            "Queen", "King",
        ];

        let mut deck = Deck::new();
        for suit in suits.iter() {
            for rank in ranks.iter() {
                deck.add_card(Card::new(rank, suit));
            }
        }

        deck
    }
}
