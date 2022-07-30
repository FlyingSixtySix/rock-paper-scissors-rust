use core::fmt;
use std::io::{self, Write};

use text_io::read;
use rand::prelude::*;
use colored::Colorize;

#[derive(Debug, PartialEq, Eq)]
enum RPSOption {
    Rock,
    Paper,
    Scissors,
    Quit,
    Invalid,
}

impl fmt::Display for RPSOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RPSOption::Rock => write!(f, "rock"),
            RPSOption::Paper => write!(f, "paper"),
            RPSOption::Scissors => write!(f, "scissors"),
            _ => Ok(())
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum RPSWinState {
    Win,
    Loss,
    Tie,
    Undetermined,
}

fn get_user_input() -> RPSOption {
    print!("\nRock, Paper, Scissors, Quit: ");
    io::stdout().flush().unwrap();

    let user_raw: String = read!();
    println!();
    return match user_raw.to_lowercase().as_str() {
        "r" | "rock" => RPSOption::Rock,
        "p" | "paper" => RPSOption::Paper,
        "s" | "scissors" => RPSOption::Scissors,
        "q" | "quit" => RPSOption::Quit,
        _ => RPSOption::Invalid,
    }
}

fn get_random_input() -> RPSOption {
    let mut rng: ThreadRng = rand::thread_rng();
    let random_pick: i8 = rng.gen_range(1..=3);
    return match random_pick {
        1 => RPSOption::Rock,
        2 => RPSOption::Paper,
        3 => RPSOption::Scissors,
        _ => RPSOption::Invalid,
    }
}

fn main() {
    println!("Rock Paper Scissors in Rust by FlyingSixtySix");

    loop {
        let user_input: RPSOption = get_user_input();
        let random_input: RPSOption = get_random_input();

        if user_input == RPSOption::Invalid {
            println!("You picked an invalid option.");
            continue;
        } else if user_input == RPSOption::Quit {
            println!("Goodbye!");
            break;
        }

        let mut win_state: RPSWinState = RPSWinState::Undetermined;

        println!("You picked {:?} and the computer picked {:?}.", user_input, random_input);
        if user_input == random_input {
            win_state = RPSWinState::Tie;
        }
        if user_input == RPSOption::Rock && random_input == RPSOption::Paper {
            // Loss; user rock < random paper
            win_state = RPSWinState::Loss;
        }
        if user_input == RPSOption::Rock && random_input == RPSOption::Scissors {
            // Win; user rock > random scissors
            win_state = RPSWinState::Win;
        }
        if user_input == RPSOption::Paper && random_input == RPSOption::Rock {
            // Win; user paper > random rock
            win_state = RPSWinState::Win;
        }
        if user_input == RPSOption::Paper && random_input == RPSOption::Scissors {
            // Loss; user paper < random scissors
            win_state = RPSWinState::Loss;
        }
        if user_input == RPSOption::Scissors && random_input == RPSOption::Rock {
            // Loss; user scissors < random rock
            win_state = RPSWinState::Loss;
        }
        if user_input == RPSOption::Scissors && random_input == RPSOption::Paper {
            // Win; user scissors > random paper
            win_state = RPSWinState::Win;
        }

        match win_state {
            RPSWinState::Win => println!("You {}! Your {} beats their {}.", "won".green(), user_input, random_input),
            RPSWinState::Loss => println!("You {}. Their {} beats your {}.", "lost".red(), random_input, user_input),
            RPSWinState::Tie => println!("You {}. You both have {}.", "tied".yellow(), user_input),
            _ => ()
        }
    }
}
