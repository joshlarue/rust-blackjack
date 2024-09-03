use crate::card::*;
use crate::deck::*;

#[derive(PartialEq, Debug)]
pub enum PlayerType {
    Dealer,
    Player1,
    Player2,
}

#[derive(Debug, PartialEq)]
pub struct Player {
    player_type: PlayerType,
    hand: Vec<Card>,
}

impl Player {
    pub fn new(player_type: PlayerType) -> Player {
        Player {
            player_type,
            hand: vec![],
        }
    }

    pub fn draw_card(&mut self, deck: &mut Deck, num_cards: u8) {
        for _ in 0..num_cards {
            self.hand.append(&mut vec![deck.deal_card()]);
        }
    }

    pub fn return_hand(self) -> Vec<Card> {
        self.hand
    }

    pub fn print_cards_in_range(&self, start: usize, end: usize) -> String {
        let mut cards = String::new();
        for i in start..end {
            cards.push_str(&format!("{}", self.hand[i]));
        }
        cards
    }

    pub fn print_hand(&self) -> String {
        self.hand
            .iter()
            .map(|card| format!("{}", card))
            .collect::<Vec<_>>()
            .join(" and ")
    }

    pub fn calculate_hand_value(&self) -> u8 {
        let mut total = 0;
        let mut aces = 0;

        for card in &self.hand {
            if card.rank == Rank::Ace {
                aces += 1
            };

            total += card.rank.clone().value();
        }

        while total > 21 && aces > 0 {
            total -= 10;
            total += 1;
            aces -= 1;
        }

        total
    }

    pub fn calculate_face_up_value(&self) -> u8 {
        self.hand[0].rank.clone().value()
    }
}

pub fn print_dealer_wins(dealer: &mut Player) {
    println!(
        "The dealer wins with a hand of {} and a total value of {}.",
        dealer.print_hand(),
        dealer.calculate_hand_value()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_player_created() {
        assert_eq!(
            Player::new(PlayerType::Player1),
            Player {
                player_type: PlayerType::Player1,
                hand: vec![]
            }
        )
    }

    #[test]
    fn draw_correct_card() {
        let mut player = Player::new(PlayerType::Player1);
        let mut deck = Deck::new();
        deck.add_card(Card::new(Suit::Hearts, Rank::Ten));
        player.draw_card(&mut deck, 1);

        assert_eq!(
            player.return_hand(),
            vec![Card::new(Suit::Hearts, Rank::Ten)]
        )
    }

    #[test]
    fn correct_hand_value() {
        let mut player = Player::new(PlayerType::Player1);
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

        player.draw_card(&mut deck, 1);
        player.draw_card(&mut deck, 1);
        player.draw_card(&mut deck, 1);
        player.draw_card(&mut deck, 1);

        assert_eq!(player.calculate_hand_value(), 14);
    }
}
