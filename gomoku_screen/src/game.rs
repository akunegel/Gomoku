use crate::rules;

pub struct Game { 
    pub board: [[i32; 19]; 19],
    pub is_black_turn: bool,
    pub black_captures: i32,
    pub white_captures: i32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: [[0; 19]; 19],
            is_black_turn: true,
            black_captures: 0,
            white_captures: 0,
        }
    }

    pub fn place_stone(&mut self, y: usize, x: usize) {
        let player = if self.is_black_turn { 1 } else { 2 };
        self.board[y][x] = player;

        let count = rules::capture::check_captures(&mut self.board, y, x);
        if self.is_black_turn {
            self.black_captures += count;
        } else {
            self.white_captures += count;
        }
        self.is_black_turn = !self.is_black_turn;
    }
}