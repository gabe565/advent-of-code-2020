use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

use super::seat::Seat;

const ROWS: i32 = 127;
const COLS: i32 = 7;
const BACK_ROW: char = 'B';
const FRONT_ROW: char = 'F';
const RIGHT_COL: char = 'R';
const LEFT_COL: char = 'L';

#[derive(Eq)]
pub struct BoardingPass {
    str: &'static str,
    pub row: i32,
    pub col: i32,
    pub seat: Seat,
}

#[allow(dead_code)]
impl BoardingPass {
    pub fn new() -> Self {
        BoardingPass {
            str: "",
            row: 0,
            col: 0,
            seat: Seat::new(0),
        }
    }

    pub fn from_str(str: &'static str) -> Self {
        let mut new = Self::new();
        new.str = str;
        new.compute();
        new
    }

    pub fn compute(&mut self) {
        let mut min_row = 0;
        let mut max_row = ROWS;
        let mut min_col = 0;
        let mut max_col = COLS;

        for i in self.str.chars() {
            match i {
                BACK_ROW => min_row = ((max_row - min_row) / 2) + min_row + 1,
                FRONT_ROW => max_row = ((max_row - min_row) / 2) + min_row,
                RIGHT_COL => min_col = ((max_col - min_col) / 2) + min_col + 1,
                LEFT_COL => max_col = ((max_col - min_col) / 2) + min_col,
                _ => panic!("invalid character in input {}", i),
            }
        }

        if min_row == max_row {
            self.row = min_row;
        } else {
            panic!("rows do not match: min {}, max {}", min_row, max_row);
        }

        if max_col == max_col {
            self.col = min_col;
        } else {
            panic!("cols do not match: min {}, max {}", min_col, max_col);
        }

        self.seat.id = (self.row * 8) + self.col;
    }
}

impl Display for BoardingPass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "row {}, column {}, {}", self.row, self.col, self.seat)
    }
}

impl PartialEq for BoardingPass {
    fn eq(&self, other: &Self) -> bool {
        self.seat == other.seat
    }
}

impl Ord for BoardingPass {
    fn cmp(&self, other: &Self) -> Ordering {
        self.seat.cmp(&other.seat)
    }
}

impl PartialOrd for BoardingPass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}