//! Random Agent
//! 
//! Uses Random chance to choose moves for connect four.
#![allow(unused)]

use std::io;
use std::io::Write;
use crate::Player;
use crate::game::*;

mod ffi {
    #[link(name = "c")]
    extern {
        pub fn rand() -> i32;
        pub fn srand(seed: u32);
        pub fn time(timer: *mut u8) -> u32;
    }
}

fn rand() -> i32
{
    unsafe{ffi::rand()}
}

fn seed_rand_with_time()
{
    unsafe{
        ffi::srand(ffi::time(std::ptr::null_mut()));
    }
}

pub struct RandomPlayer;

impl RandomPlayer {
    pub fn new() -> Self {
        // unsafe{ffi::srand(3)};
        seed_rand_with_time();
        RandomPlayer
    }
}

impl Player for RandomPlayer {
    fn choose_move(&self, board: &Board, disc: Disc) -> u8 {
        let options = (0..COLUMN_COUNT as u8).filter(
            |&col| !board.is_column_full(col)
        ).collect::<Vec<_>>();
        let chosen = options[(rand() as usize % options.len())];
        println!("Random move chosen (from set of {} moves)", options.len());  
        chosen
    }
    fn get_name(&self) -> &str {
        "Random"
    }
}
