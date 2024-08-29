mod deck;
use deck::deck::new_deck;

fn main() {
    let deck = new_deck();
    println!("{:?}", deck);
}
