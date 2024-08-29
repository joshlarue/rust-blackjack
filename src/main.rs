mod deck;
mod player;
use deck::new_deck;

fn main() {
    let deck = new_deck();
    println!("{:?}", deck);
}
