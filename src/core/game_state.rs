use super::rules::{capture, win, double_three};

#[derive(Clone)]
pub struct GameState {
    pub board: [[u8; 19]; 19],
    pub captures: [u32; 2],
    pub turn_count: u32,
    pub winner: Option<u8>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            board: [[0; 19]; 19],
            captures: [0, 0],
            turn_count: 0,
            winner: None,
        }
    }

    pub fn current_player(&self) -> u8 {
        ((self.turn_count % 2) + 1) as u8
    }

    pub fn can_place_piece(&self, x: usize, y: usize) -> Result<(), String> {
        if !capture::is_in_board(y as i32, x as i32) {
            return Err("Outside the board".into());
        }
        if self.board[y][x] != 0 {
            return Err("Already occupied".into());
        }

        let player = self.current_player();
        if double_three::is_double_three(&self.board, y, x, player) {
            return Err("Forbidden: Double Three".into());
        }

        Ok(())
    }

    pub fn place_piece(&mut self, x: usize, y: usize) {
        let player = self.current_player();
        self.board[y][x] = player;

        let captured = capture::apply_captures(&mut self.board, y, x);
        self.captures[(player - 1) as usize] += captured;

        self.turn_count += 1;
    }

    pub fn check_win(&self) -> Option<u8> {
        win::check_win(&self.board, &self.captures)
    }
}