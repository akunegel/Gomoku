#[derive(Clone)]
pub struct GameState {
    pub board: [[u8; 19]; 19],
    pub captures: [u32; 2],
    pub turn_count: u32,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            board: [[0; 19]; 19],
            captures: [0, 0],
            turn_count: 0,
        }
    }

    pub fn place_piece(&mut self, x: usize, y: usize) {
        let player = (self.turn_count % 2) + 1;
        self.board[y][x] = player as u8;
        self.turn_count += 1;
    }

    pub fn current_player(&self) -> u8 {
        ((self.turn_count % 2) + 1) as u8
    }
}
