use crate::{card, deck, player::*};
use std::io::{self, Error};

// restructure to have create_1_player func and create 2 func based on another func that takes
// determines num_players
pub fn create_one_player() -> Result<(Player, Player), Error> {
    let dealer = Player::new(PlayerType::Dealer);
    let player1 = Player::new(PlayerType::Player1);

    Ok((dealer, player1))
}

/// sets up first round for 1 player mode by dealing 2 cards to dealer and 2 cards to player
/// prints cards and values for dealer's first card and player's cards
pub fn one_player_first_round(deck: &mut deck::Deck, dealer: &mut Player, player1: &mut Player) {
    dealer.draw_card(deck, 2);

    player1.draw_card(deck, 2);
    println!(
        "You have {} and your hand is worth {}.",
        player1.print_hand(),
        player1.calculate_hand_value()
    );
    println!(
        "The dealer's face-up card is {} and is worth {}.",
        dealer.print_cards_in_range(0, 1),
        dealer.calculate_face_up_value()
    );
}

pub fn hit_or_stay() -> Result<bool, Error> {
    loop {
        println!("Type 'h' to hit or 's' to stay.");

        let mut io_buf = String::new();
        io::stdin().read_line(&mut io_buf)?;

        // need to call .trim() so that the input doesn't include the \n
        match io_buf.as_str().trim() {
            "h" => {
                // this print statement clears the screen
                print!("\x1B[2J\x1B[1;1H");
                break Ok(true);
            }
            "s" => {
                print!("\x1B[2J\x1B[1;1H");
                break Ok(false);
            }
            _ => continue,
        }
    }
}

pub fn play_again() -> Result<bool, Error> {
    loop {
        println!("\nPlay again? (y/n)");

        let mut io_buf = String::new();
        io::stdin().read_line(&mut io_buf)?;

        // need to call .trim() so that the input doesn't include the \n
        match io_buf.as_str().trim() {
            "y" => {
                // this print statement clears the screen
                print!("\x1B[2J\x1B[1;1H");
                break Ok(true);
            }
            "n" => {
                print!("\x1B[2J\x1B[1;1H");
                break Ok(false);
            }
            _ => continue,
        }
    }
}

/// draws one card for specified player and prints hand/value for player and dealer
/// returns true/false for whether or not the player busted during the hit
pub fn player_hit(deck: &mut deck::Deck, dealer: &mut Player, player1: &mut Player) -> bool {
    player1.draw_card(deck, 1);

    if busted(player1) {
        determine_winner(deck, dealer, &*player1);
        return true;
    } else {
        println!(
            "You have {} and your hand is worth {}.",
            player1.print_hand(),
            player1.calculate_hand_value()
        );
        println!(
            "The dealer's face-up card is {} and is worth {}.",
            dealer.print_cards_in_range(0, 1),
            dealer.calculate_face_up_value(),
        );
        return false;
    }
}

pub fn busted(player: &Player) -> bool {
    return player.calculate_hand_value() > 21;
}

pub fn blackjack(player: &Player) -> bool {
    return player.calculate_hand_value() == 21;
}

pub fn determine_winner(deck: &mut deck::Deck, dealer: &mut Player, player1: &Player) {
    let mut dealer_busted = false;
    let player_busted = busted(player1);

    while dealer.calculate_hand_value() < 17 {
        dealer.draw_card(deck, 1);
        if busted(dealer) {
            dealer_busted = true;
        }
    }

    // these returns don't return out of the function :(
    if player_busted && dealer_busted {
        println!("PUSHED! You and the dealer both busted!");
        println!(
            "You had a hand of {} and a total value of {}.",
            player1.print_hand(),
            player1.calculate_hand_value()
        );
        println!(
            "The dealer had a hand of {} and a total value of {}.",
            dealer.print_hand(),
            dealer.calculate_hand_value()
        );
        return;
    } else if dealer_busted {
        println!("The dealer busted!");
        println!(
            "You win with a hand of {} and a total value of {}.",
            player1.print_hand(),
            player1.calculate_hand_value()
        );
        println!(
            "The dealer had a hand of {} and a total value of {}.",
            dealer.print_hand(),
            dealer.calculate_hand_value()
        );
        return;
    } else if player_busted {
        println!("You busted!");
        println!(
            "You had a hand of {} and a total value of {}.",
            player1.print_hand(),
            player1.calculate_hand_value()
        );
        println!(
            "The dealer had a hand of {} and a total value of {}.",
            dealer.print_hand(),
            dealer.calculate_hand_value()
        );
        return;
    }

    let dealer_hand_value = dealer.calculate_hand_value();
    let player_hand_value = player1.calculate_hand_value();

    let player_blackjack = blackjack(player1);
    let dealer_blackjack = blackjack(dealer);

    if player_hand_value != dealer_hand_value {
        if player_blackjack {
            println!("You blackjacked!");
        } else if dealer_blackjack {
            println!("The dealer blackjacked!");
        }
    }

    if player_hand_value <= 21 && player_hand_value > dealer_hand_value {
        println!(
            "You win with a hand of {} and a total value of {}.",
            player1.print_hand(),
            player1.calculate_hand_value()
        );
        println!(
            "The dealer had a hand of {} and a total value of {}.",
            dealer.print_hand(),
            dealer.calculate_hand_value()
        );
    } else if player_hand_value == dealer_hand_value {
        if player_blackjack {
            println!("PUSHED! Y'all both have a Blackjack!");
        } else {
            println!("PUSHED! Y'all have the same hand value!");
        }
        println!(
            "You had a hand of {} and a total value of {}.",
            player1.print_hand(),
            player1.calculate_hand_value()
        );
        println!(
            "The dealer had a hand of {} and a total value of {}.",
            dealer.print_hand(),
            dealer.calculate_hand_value()
        );
    } else {
        println!(
            "The dealer wins with a hand of {} and a total value of {}.",
            dealer.print_hand(),
            dealer.calculate_hand_value()
        );
        println!(
            "You had a hand of {} and a total value of {}.",
            player1.print_hand(),
            player1.calculate_hand_value()
        );
    }
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
            1 => {
                // this line clears the console screen
                print!("\x1B[2J\x1B[1;1H");
                break Ok(1);
            }
            2 => {
                break {
                    print!("\x1B[2J\x1B[1;1H");
                    Ok(2)
                }
            }
            _ => continue,
        }
    }
}
