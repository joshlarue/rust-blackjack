mod card;
mod deck;
mod game_logic;
mod player;
use deck::new_deck;

fn main() {
    let deck = new_deck();
    println!("{:?}", deck);
}
