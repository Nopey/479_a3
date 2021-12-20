//! Human player for Connect Four
//! 
//! Uses stdin and stdout to prompt a human for moves.
#![allow(unused)]

use std::io;
use std::io::Write;
use crate::Player;
use crate::game::*;

pub struct HumanPlayer {
    name: String,
}

impl HumanPlayer {
    pub fn new() -> Self {
        let mut name = String::new();
        print!("Enter your name, player: ");
        io::stdout().flush().ok();
        let _read = io::stdin().read_line(&mut name);
        name = name.trim().to_owned();
        HumanPlayer {
            name
        }
    }
}

impl Player for HumanPlayer {
    fn choose_move(&self, _board: &Board, disc: Disc) -> u8 {
        print!("{}, Choose a column (1..={}) to drop your {} piece: ",
            self.name, crate::game::COLUMN_COUNT, disc);
        io::stdout().flush().ok();
        let mut input = String::new();
        let _read = io::stdin().read_line(&mut input);
        input = input.trim().to_owned();
        input.parse::<u8>().unwrap_or(0).wrapping_sub(1)
    }
    fn get_name(&self) -> &str {
        &self.name
    }
}
