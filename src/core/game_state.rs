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
        let captured = self.perform_captures(y, x);
        self.captures[(player - 1) as usize] += captured;
        self.turn_count += 1;
    }

    fn perform_captures(&mut self, y: usize, x: usize) -> u32 {
        let mut count = 0;
        let player = self.board[y][x];
        let opponent = if player == 1 { 2 } else { 1 };

        let directions = [
            (0, 1), (0, -1), (1, 0), (-1, 0),
            (1, 1), (1, -1), (-1, 1), (-1, -1)
        ];
        for (dy, dx) in directions.iter() {
            let y1 = y as i32 + dy;
            let x1 = x as i32 + dx;
            let y2 = y as i32 + dy * 2;
            let x2 = x as i32 + dx * 2;
            let y3 = y as i32 + dy * 3;
            let x3 = x as i32 + dx * 3;

            if self.is_in_board(y3, x3) {
                let s1 = self.board[y1 as usize][x1 as usize];
                let s2 = self.board[y2 as usize][x2 as usize];
                let s3 = self.board[y3 as usize][x3 as usize];

                if s1 == opponent && s2 == opponent && s3 == player {
                    self.board[y1 as usize][x1 as usize] = 0;
                    self.board[y2 as usize][x2 as usize] = 0;
                    count += 2;
                }
            }
        }
        count
    }
    
    fn is_in_board(&self, y: i32, x: i32) -> bool {
        x >= 0 && x < 19 && y >= 0 && y < 19
    }
    pub fn current_player(&self) -> u8 {
        ((self.turn_count % 2) + 1) as u8
    }
}
