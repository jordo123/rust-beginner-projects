
use rand::seq::SliceRandom;
use std::cmp::{Ord, Ordering};
use std::fmt;
use std::io::{self, Write};

#[derive(Eq)]
pub struct RPS {
    /// The chosen move: either rock, paper, or scissors
    chosen: String,
}

impl RPS {
    
    pub const ROCK: &'static str = "rock";

    pub const PAPER: &'static str = "paper";
    pub const SCISSORS: &'static str = "scissors";

    pub fn new(chosen: String) -> Option<RPS> {
        if [RPS::ROCK, RPS::PAPER, RPS::SCISSORS].contains(&chosen.as_ref()) {
            return Some(RPS { chosen });
        }
        None
    }

    pub fn default() -> RPS {
        let mut rng = rand::thread_rng();
        RPS {
            chosen: [RPS::ROCK, RPS::PAPER, RPS::SCISSORS]
                .choose(&mut rng)
                .unwrap()
                .to_string(),
        }
    }

    pub fn get_input(&mut self) -> bool {
        let mut player_move = String::new();
        print!("Rock, Paper, or scissors? [\"q\" to quit]: ");
        // Make sure `print!` actually prints to screen
        io::stdout().flush().unwrap();
        // Get user input
        io::stdin().read_line(&mut player_move).unwrap();
        // Clean up the user's input
        let player_move = player_move.trim().to_lowercase();
        // Make sure it is rock, paper, or scissors
        if [RPS::ROCK, RPS::PAPER, RPS::SCISSORS].contains(&player_move.as_ref()) {
            self.chosen = player_move;
            true
        } else if player_move == "q" {
            std::process::exit(0);
        } else {
            false
        }
    }
}

impl fmt::Display for RPS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.chosen)
    }
}

impl Ord for RPS {
    fn cmp(&self, other: &RPS) -> Ordering {
        if self.chosen == RPS::ROCK {
            if other.chosen == RPS::ROCK {
                return Ordering::Equal;
            } else if other.chosen == RPS::PAPER {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        }
        if self.chosen == RPS::PAPER {
            if other.chosen == RPS::ROCK {
                return Ordering::Greater;
            } else if other.chosen == RPS::PAPER {
                return Ordering::Equal;
            } else {
                return Ordering::Less;
            }
        }
        if self.chosen == RPS::SCISSORS {
            if other.chosen == RPS::ROCK {
                return Ordering::Less;
            } else if other.chosen == RPS::PAPER {
                return Ordering::Greater;
            } else {
                return Ordering::Equal;
            }
        }
        return Ordering::Equal;
    }
}

impl PartialOrd for RPS {
    fn partial_cmp(&self, other: &RPS) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for RPS {
    fn eq(&self, other: &RPS) -> bool {
        self.chosen == other.chosen
    }
}

pub fn play() {
    // The point tally for the computer
    let mut c_tally = 0;
    // The point tally for the player
    let mut p_tally = 0;
    // Constants so I don't have to write "rock", "paper", and "scissors" everywhere
    loop {
        let mut player_move = RPS::default();
        // Get input from user
        if !player_move.get_input() {
            println!(
                "Invalid move: must be one of \"rock\", \"paper\", or \"scissors\" [\"q\" to quit]"
            );
            continue;
        }
        // Get a random choice for the computer
        let computer = RPS::default();
        // This is allowed because RPS implements the `Display` trait
        println!("Computer: {}\nPlayer: {}", computer, player_move);
        // If the computer's move beat the player's move, it gets a point!
        if computer > player_move {
            println!("Computer Wins!");
            c_tally += 1;
        } else if computer < player_move {
            println!("Player wins!");
            p_tally += 1;
        } else {
            println!("Tie!");
        }
        // Print the score
        println!("Score Computer : {}, Player : {}", c_tally, p_tally);
    }
}
