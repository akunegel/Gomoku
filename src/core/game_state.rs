use crate::core::captures;

pub struct GameState {
    pub board: [[i32; 19]; 19],
    pub is_black_turn: bool,
    pub black_captures: i32,
    pub white_captures: i32,
    pub winner: i32,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            board: [[0; 19]; 19],
            is_black_turn: true,
            black_captures: 0,
            white_captures: 0,
            winner: 0,
        }
    }

    pub fn place_stone(&mut self, y: usize, x: usize) {
        if self.winner != 0 || self.board[y][x] != 0 { return; }

        let player = if self.is_black_turn { 1 } else { 2 };
        self.board[y][x] = player;

        // captures.rs の関数を呼び出す
        let count = captures::check_captures(&mut self.board, y, x);
        
        if self.is_black_turn { self.black_captures += count; } 
        else { self.white_captures += count; }

        // TODO: ここに勝利判定 (victory.rs) を追加する
        
        self.is_black_turn = !self.is_black_turn;
    }
}