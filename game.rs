//! Connect Four game
//! 
//! Connect Four game, successor function, and win detection
use std::fmt;
use std::convert::TryInto;

pub const COLUMN_COUNT: usize = 7;
pub const ROW_COUNT: usize = 6;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Disc {
    Red,
    Blue,
}

impl Disc {
    pub fn opposite(self) -> Self {
        use Disc::*;
        match self {
            Red  => Blue,
            Blue => Red,
        }
    }
}

impl fmt::Display for Disc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Disc::Red  => "\x1b[31mX\x1b[0m",
            Disc::Blue => "\x1b[34mO\x1b[0m",
        })
    }
}

pub enum WinState {
    /// Player won by placing Disc.
    PlayerWon(Disc),
    /// Neither player has won yet (Gupi)
    Ongoing,
    /// Ran out of places to put the discs
    Draw,
}

#[derive(Debug, Clone, Default)]
struct Column {
    discs: [Option<Disc>; ROW_COUNT],
    fullness: u8
}

impl Column {
    fn drop_disc(&mut self, disc: Disc) -> Option<u8> {
        let idx = self.fullness;
        *self.discs.get_mut(idx as usize)? = Some(disc);
        self.fullness += 1;
        Some(idx)
    }

    fn is_full(&self) -> bool {
        self.fullness as usize >= ROW_COUNT
    }
}

#[derive(Debug, Clone, Default)]
pub struct Board {
    columns: [Column; COLUMN_COUNT],
}

impl Board {
    pub fn drop_disc(&mut self, column: u8, disc: Disc) -> Option<WinState> {
        let row = self.columns.get_mut(column as usize)?.drop_disc(disc)?;
        let win_state = if self.move_wins( column, row, disc ){
            WinState::PlayerWon(disc)
        } else if self.columns.iter().all(Column::is_full) {
            WinState::Draw
        } else {
            WinState::Ongoing
        };

        Some(win_state)
    }

    pub fn is_column_full(&self, column: u8) -> bool {
        self.columns.get(column as usize)
            .map(Column::is_full)
            .unwrap_or(true)
    }

    fn move_wins( &self, column: u8, row: u8, disc: Disc ) -> bool {
        for (x_off, y_off) in &[
            (1, 0),
            (0, 1),
            (1, 1),
            (1,-1),
        ] {
            // the piece we're placing is, of course, our color.
            let mut score = 1;

            for dir in &[1i8, -1] {
                // for each of the three discs in that direction
                let mut c = column as i8;
                let mut r = row as i8;
                for _ in 0..3 {
                    c += x_off*dir;
                    r += y_off*dir;

                    if self.get(c, r) != Some(disc) {
                        // piece mismatches, so no more score in this dir
                        break;
                    }

                    // another matching piece!
                    score += 1;

                    // four in a row to win.
                    if score == 4 {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn estimate_streak_score( &self, streak: u32, positive: bool, score: &mut u32 ) {

        // fun table to play with:
        // The weight of each 
        let impact = match streak {
            3 => 8,
            2 => 4,
            1 => 1,
            _ => 0,
        };

        if positive {
            *score = score.saturating_add(impact);
        } else {
            *score = score.saturating_sub(impact);
        }
    }

    fn estimate_line_score(&self, disc: Disc, pos: [i8; 2], delta: [i8; 2], score: &mut u32 ) {
        let mut streak = 0;
        let mut streak_startedopen = false;
        let mut streak_disc = None;
        let [mut c, mut r] = pos;
        loop {
            match self.get(c, r) {
                Some(d) if streak==0 || Some(d)==streak_disc => {
                    streak += 1;
                    streak_disc = Some(d);
                },
                cell => {
                    if streak_startedopen || cell.is_none() {
                        self.estimate_streak_score(
                            streak, streak_disc == Some(disc), score );
                    }
                    streak = 0;
                    streak_startedopen = cell.is_none();
                }
            }
            // offset to the next cell
            c += delta[0];
            r += delta[1];
            // stop once we're off the board
            if c < 0 || c > COLUMN_COUNT as i8
            || r < 0 || r > ROW_COUNT as i8 {
                // apply running score
                if streak_startedopen {
                    self.estimate_streak_score(
                        streak, streak_disc == Some(disc), score );
                }
                return;
            }
        }
    }

    /// Estimate a quality heuristic of the board state
    ///   for the given player.
    ///
    /// used by minimax
    pub fn estimate_score( &self, disc: Disc ) -> u32 {
        let mut score = crate::minimax::AVG_SCORE;
        for r in 0..ROW_COUNT as i8 {
            self.estimate_line_score(disc, [0, r], [1, 0], &mut score);
            self.estimate_line_score(disc, [0, r], [1, 1], &mut score);
            if r != 0 {
                // opposite diagonal
                self.estimate_line_score(disc, [COLUMN_COUNT as i8 - 1, r], [-1,1], &mut score);
            }
        }
        for c in 0..COLUMN_COUNT as i8 {
            self.estimate_line_score(disc, [c, 0], [0, 1], &mut score);
            if c != 0 {
                self.estimate_line_score(disc, [c, 0], [1, 1], &mut score);

                // opposite diagonal
                self.estimate_line_score(disc, [c, 0], [-1, 1], &mut score);
            }
        }
        // clamp score estimate to 1..(MAX_SCORE-1)
        if score == 0 {
            1
        } else if score >= crate::minimax::MAX_SCORE {
            crate::minimax::MAX_SCORE - 1
        } else {
            score
        }
    }

    fn get( &self, column: i8, row: i8 ) -> Option<Disc> {
        *self.columns.get::<usize>(column.try_into().ok()?)?
            .discs.get::<usize>(row.try_into().ok()?)?
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in 1..=COLUMN_COUNT {
            write!(f, "_\x1b[90m{}\x1b[0m", c)?;
        }
        writeln!(f, "_")?;
        for row in (0..ROW_COUNT).rev() {
            for column in &self.columns {
                let disc = column.discs[row];
                match disc {
                    Some(d) => write!(f, "|{}", d)?,
                    None => write!(f, "| ")?
                };
            }
            writeln!(f, "|")?
        }
        Ok(())
    }
}
