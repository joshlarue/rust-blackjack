use crate::{card, deck, player::*};
use std::io::{self, Error};

// restructure to have create_1_player func and create 2 func based on another func that takes
// determines num_players
pub fn create_one_player() -> Result<(Player, Player), Error> {
    let dealer = Player::new(PlayerType::Dealer);
    let player1 = Player::new(PlayerType::Player1);

    Ok((dealer, player1))
}

pub fn play_1_player_round(deck: &mut deck::Deck, dealer: &mut Player, player1: &mut Player) {
    dealer.draw_card(deck);
    dealer.draw_card(deck);

    player1.draw_card(deck);
    player1.draw_card(deck);
    println!(
        "You have {} and your hand is worth {}.",
        player1.print_hand(),
        player1.calculate_hand_value()
    );
    println!(
        "The dealer's face-up card is {} and is worth {}.",
        dealer.print_first_card(),
        dealer.calculate_hand_value()
    );
    println!("Would you like to hit or stay?");
}

pub fn get_num_players() -> Result<i8, Error> {
    loop {
        println!("Type Ctrl-C or Cmd-C to quit.");
        println!("Type 1 for one player and 2 for two players.");

        let mut io_buf = String::new();
        io::stdin().read_line(&mut io_buf)?;

        let number_input = io_buf.trim_end().parse::<i8>();
        println!("\n\n");
        let number_res = match number_input {
            Ok(num) => num,
            Err(_) => -1,
        };

        match number_res {
            1 => break Ok(1),
            2 => break Ok(2),
            _ => continue,
        }
    }
}
