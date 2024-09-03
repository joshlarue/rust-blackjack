mod card;
mod deck;
mod game_logic;
mod player;
use deck::*;
use game_logic::*;
use std::io;

fn main() -> Result<(), io::Error> {
    let mut deck = new_deck();
    // this print statement clears the screen
    print!("\x1B[2J\x1B[1;1H");

    loop {
        let num_players = get_num_players()?;

        if num_players == 1 {
            let (mut dealer, mut player1) = match create_one_player() {
                Ok(players) => players,
                Err(_) => panic!("Player creation went horribly wrong!"),
            };

            one_player_first_round(&mut deck, &mut dealer, &mut player1);

            loop {
                let hit = hit_or_stay()?;
                if hit {
                    let busted = player_hit(&mut deck, &mut dealer, &mut player1);
                    if busted {
                        break;
                    }
                } else {
                    determine_winner(&mut deck, &mut dealer, &player1);
                    break;
                }
            }
            if !play_again()? {
                break;
            }
        }
    }

    Ok(())
}
