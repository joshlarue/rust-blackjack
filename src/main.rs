mod card;
mod deck;
mod game_logic;
mod player;
use std::io;

use deck::new_deck;
use game_logic::*;

fn main() -> Result<(), io::Error> {
    let mut deck = new_deck();

    let num_players = get_num_players()?;

    if num_players == 1 {
        let (mut dealer, mut player1) = match create_one_player() {
            Ok(players) => players,
            Err(_) => panic!("Player creation went horribly wrong!"),
        };
        play_1_player_round(&mut deck, &mut dealer, &mut player1);
    }

    Ok(())
}
