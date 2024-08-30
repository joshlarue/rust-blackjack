use crate::deck::*;

#[derive(Debug, PartialEq)]
pub struct Player {
    name: String,
    hand: Vec<Card>,
}
impl Player {
    pub fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
            hand: vec![],
        }
    }

    pub fn draw_card(&mut self, deck: &mut Deck) {
        self.hand.push(deck.deal_card());
    }

    pub fn show_hand(self) -> Vec<Card> {
        self.hand
    }

    pub fn calculate_hand_value(self) -> u8 {
        let mut total = 0;
        let mut aces = 0;

        for card in self.hand {
            println!("{:?}", card);
            if card.rank == Rank::Ace {
                aces += 1
            };

            total += card.rank.value();
        }

        while total > 21 && aces > 0 {
            total -= 10;
            total += 1;
            aces -= 1;
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_player_created() {
        assert_eq!(
            Player::new("player"),
            Player {
                name: "player".to_string(),
                hand: vec![]
            }
        )
    }

    #[test]
    fn draw_correct_card() {
        let mut player = Player::new("player");
        let mut deck = Deck::new();
        deck.add_card(Card::new(Suit::Hearts, Rank::Ten));
        player.draw_card(&mut deck);

        assert_eq!(player.show_hand(), vec![Card::new(Suit::Hearts, Rank::Ten)])
    }

    #[test]
    fn correct_hand_value() {
        let mut player = Player::new("player");
        let mut deck = Deck::new();
        deck.add_card(Card {
            suit: Suit::Hearts,
            rank: Rank::Ten,
        });
        deck.add_card(Card {
            suit: Suit::Spades,
            rank: Rank::Ace,
        });
        deck.add_card(Card {
            suit: Suit::Clubs,
            rank: Rank::Ace,
        });
        deck.add_card(Card {
            suit: Suit::Diamonds,
            rank: Rank::Two,
        });

        player.draw_card(&mut deck);
        player.draw_card(&mut deck);
        player.draw_card(&mut deck);
        player.draw_card(&mut deck);

        assert_eq!(player.calculate_hand_value(), 14);
    }
}
