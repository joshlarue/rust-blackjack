use crate::deck::{Card, Deck};

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

    pub fn show_hand(self) {
        println!("{:?}", self.hand);
    }

    pub fn calculate_hand_value(self) -> i32 {
        let mut total = 0;
        let mut aces = 0;

        for card in self.hand {
            match card.rank.as_str() {
                "Ace" => aces += 1,
                "Ten" | "Queen" | "King" => total += 10,
                _ => total += card.rank.parse::<i32>().unwrap(),
            }
        }

        while total >= 21 && aces > 0 {
            total -= 10;
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
}
