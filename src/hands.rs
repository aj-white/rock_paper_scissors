use std::io::{stdin, stdout, Write};
use rand::{thread_rng, Rng};

#[derive(Debug, Copy, Clone)]
pub enum Hand {
    Rock,
    Paper,
    Scissors
}

impl Hand {
    pub fn get_random() -> Self {
        match thread_rng().gen_range(1..=3) {
            1 => Hand::Rock,
            2 => Hand::Paper,
            3 => Hand::Scissors,
            _ => unreachable!(),
        }
    }

    pub fn user_choice() -> Self {
        let choice = loop {
            println!("Enter choice [(r)ock, (p)aper, (S)cissors] or 'q' to quit: ");
            let mut input = String::new();
            let _ = stdout().flush();
            stdin().read_line(&mut input).unwrap();

            let user_entry = match input.trim().to_lowercase().as_str() {
                "r" | "rock" => Hand::Rock,
                "p" | "paper" => Hand::Paper,
                "s" | "scissors" => Hand::Scissors,
                "q" | "quit" => std::process::exit(0),
                _ => continue
            };
            break user_entry
        };
        choice
    }
}