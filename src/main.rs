mod card;
mod deck;
mod game_logic;
mod player;
use std::io;

fn main() -> Result<(), io::Error> {
    let mut deck = new_deck();
    println!("\u{1F0A1}");

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
                player_hit(&mut deck, &mut player1);
                if busted(&mut player1) {
                    determine_winner(&mut deck, &mut dealer, &mut player1);
                    break;
                };
                continue;
            } else {
                determine_winner(&mut deck, &mut dealer, &mut player1);
                break;
            }
        }
    }

    Ok(())
}
