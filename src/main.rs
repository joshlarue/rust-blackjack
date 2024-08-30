mod card;
mod deck;
mod game_logic;
mod player;
use deck::new_deck;
use game_logic::create_players;

fn main() {
    let deck = new_deck();
    println!("{:?}", deck);
    let _ = create_players();
}
