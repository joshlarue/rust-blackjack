use crate::{card, deck, player::*};
use std::io::{self};

pub fn create_game() -> Result<()> {
    let dealer = Player::new(PlayerType::Dealer);

    let mut valid_input = false;
    let mut number_input;

    loop {
        println!("Type 1 for one player and 2 for two players.");

        let mut io_buf = String::new();
        io::stdin().read_line(&mut io_buf)?;

        let number_input = io_buf.trim_end().parse::<u8>();
        match number_input {
            Ok(num) => num,
            _ => eprintln!("Come on, give me a number you silly goose."),
        }
    }

    let player = Player::new(PlayerType::Dealer);
}
