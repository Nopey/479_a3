//! fourfight
//!
//! This file contains the entrypoint
//! 
//! Magnus Larsen 2021

mod game;

mod human;
mod minimax;
mod random;

use game::*;

pub trait Player {
    fn choose_move(&self, board: &Board, disc: Disc) -> u8;
    fn get_name(&self) -> &str;
}

/// 
pub fn play_game<'a>( red_player: &'a dyn Player, blue_player: &'a dyn Player ) {
    let mut players = [(blue_player, Disc::Blue), (red_player, Disc::Red)];
    let mut board: Board = Default::default();

/*
    // Optionally apply N rounds of random gameplay
    let rand = random::RandomPlayer::new();
    for _ in 0..3 {
        board.drop_disc(rand.choose_move(&board, Disc::Red),  Disc::Red);
        board.drop_disc(rand.choose_move(&board, Disc::Blue), Disc::Blue);
    }
*/

    print!("{}", board);
    loop{
        players.swap(0, 1);
        let (ref player, disc) = players[0];
        let column = player.choose_move(&board, disc);
        let state = if let Some(state) = board.drop_disc(column, disc) {
            state
        } else {
            println!("{} Player, {}, is cheating!", disc, player.get_name());
            return;
        };
        print!("\n{}", board);
        match state {
            WinState::Draw => {
                println!("Game drawn");
                return;
            },
            WinState::PlayerWon(winning_player) => {
                assert_eq!(winning_player, disc);
                println!("{} Player, {}, won!", disc, player.get_name());
                return; 
            },
            WinState::Ongoing => (),
        }
    }
}


/// Handles commandline interface and program lifecycle
fn main() {
    println!("\x1b[1m-= C\x1b[34mO\x1b[39mNNECT \x1b[31m4\x1b[39m =-\x1b[0m");
    // First player listed goes first, places X.
    // Second player listed goes second, places O.
    play_game(
        // NOTE: The parameter to the 'MinimaxPlayer::new' function is the ply.
        //&mut random::RandomPlayer::new()
        //&mut random::RandomPlayer::new(),
        &mut minimax::MinimaxPlayer::new(6),
        //&mut minimax::MinimaxPlayer::new(6),
        &mut human::HumanPlayer::new(),
    );
}
