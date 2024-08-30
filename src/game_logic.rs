use crate::{card, deck, player::*};
use std::io::{self, Error};

pub fn create_players() -> Result<(Player, Player, Player, i8), Error> {
    let num_players = loop {
        println!("Type 1 for one player and 2 for two players.");
        println!("Type Ctrl-C or Cmd-C to quit.");

        let mut io_buf = String::new();
        io::stdin().read_line(&mut io_buf)?;

        let number_input = io_buf.trim_end().parse::<i8>();
        let number_res = match number_input {
            Ok(num) => num,
            Err(_) => -1,
        };

        if number_res != -1 && number_res == 1 || number_res == 2 {
            match number_res {
                1 => break 1,
                2 => break 2,
                _ => continue,
            }
        }
    };

    let dealer = Player::new(PlayerType::Dealer);
    let player1 = Player::new(PlayerType::Player1);
    let player2 = Player::new(PlayerType::Player2);

    Ok((dealer, player1, player2, num_players))
}
