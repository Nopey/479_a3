//! Connect Four Minimax Agent
//! 
//! Uses minimax algorithm to choose moves for connect four.
#![allow(unused)]

use std::io;
use std::io::Write;
use std::u32;
use crate::Player;
use crate::game::*;

pub struct MinimaxPlayer{
    ply: u32
}

impl MinimaxPlayer {
    pub fn new(ply: u32) -> Self {
        MinimaxPlayer{ply}
    }
}

impl Player for MinimaxPlayer {
    fn choose_move(&self, board: &Board, disc: Disc) -> u8 {
        let choice = minimax(board, disc, 0, MAX_SCORE, self.ply);
        println!(
            "{}'s minimax move found in {} steps with score of {} (~{}%, \u{0394}{})",
            disc,
            choice.steps,
            choice.score,
            (choice.score as f32) / (MAX_SCORE as f32) * 100.,
            choice.score as i32 - AVG_SCORE as i32
        );
        choice.column
    }
    fn get_name(&self) -> &str {
        "Minimax"
    }
}

struct MinimaxResult{
    score: u32,
    column: u8,
    steps: u32,
}

/// the largest even number
///
/// Why even? so that `MAX_SCORE/2 == AVG_SCORE == MAX_SCORE - AVG_SCORE`
pub const MAX_SCORE: u32 = u32::MAX;
/// mean score
pub const AVG_SCORE: u32 = MAX_SCORE/2;


/// disc indicates which disc needs to be placed next
/// alpha: current highest score
/// beta: current lowest score
/// patience: the remaining recursion depth before early-termination
fn minimax( board: &Board, disc: Disc, mut alpha: u32, beta: u32, patience: u32 ) -> MinimaxResult {
    if patience == 0 {
        return MinimaxResult{
            score: board.estimate_score(disc),
            column: 255,
            steps: 1
        };
    }

    let mut best: MinimaxResult = MinimaxResult{
        score: 0,
        column: 255, // trashy invalid column
        steps: 1
    };

    for column in  0..COLUMN_COUNT as u8 {
        // drop piece to column
        let mut next_board = board.clone();
        let win_state = match next_board.drop_disc(column, disc) {
            Some(ws) => ws,
            // column is full
            None => continue
        };

        // score terminal states
        use WinState::*;
        let score = match win_state {
            WinState::PlayerWon(winner) => MAX_SCORE,
            Draw => AVG_SCORE,
            Ongoing => {
                let opponent = minimax(
                    &next_board,
                    disc.opposite(),
                    MAX_SCORE - beta,
                    MAX_SCORE - alpha,
                    patience - 1
                );
                best.steps += opponent.steps;
                MAX_SCORE - opponent.score
            },
        };

        if score >= best.score {
            best = MinimaxResult {
                score,
                column,
                steps: best.steps,
            };

            if score > beta {
                return best;
            }

            alpha = std::cmp::max(score, alpha);
        }
    }

    if best.column == 255 {
        panic!("minimax called on drawn game: \n{}", board);
    }

    best
}
