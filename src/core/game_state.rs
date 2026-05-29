const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (1, 1), (1, -1)];

#[derive(Clone)]
pub struct GameState {
    pub board: [[u8; 19]; 19],
    pub captures: [u32; 2],
    pub turn_count: u32,
    pub moves: Vec<(usize, usize, u8)>,
    history: Vec<GameState>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            board: [[0; 19]; 19],
            captures: [0, 0],
            turn_count: 0,
            moves: Vec::new(),
            history: Vec::new(),
        }
    }

    pub fn place_piece(&mut self, x: usize, y: usize) {
        self.history.push(self.clone());
        if self.history.len() > 3 {
            self.history.remove(0);
        }
        let player = ((self.turn_count % 2) + 1) as u8;
        self.board[y][x] = player;
        self.moves.push((x, y, player));
        self.turn_count += 1;
    }

    pub fn undo(&mut self) {
        if let Some(prev) = self.history.pop() {
            *self = prev;
        }
    }

    pub fn check_win(&self) -> Option<u8> {
        for y in 0..19 {
            for x in 0..19 {
                let player = self.board[y][x];
                if player == 0 {
                    continue;
                }
                for &(dx, dy) in &DIRECTIONS {
                    let mut count = 1;
                    for step in 1..5 {
                        let nx = x as isize + dx * step;
                        let ny = y as isize + dy * step;
                        if nx < 0 || nx >= 19 || ny < 0 || ny >= 19 {
                            break;
                        }
                        if self.board[ny as usize][nx as usize] != player {
                            break;
                        }
                        count += 1;
                    }
                    if count == 5 {
                        return Some(player);
                    }
                }
            }
        }
        None
    }
}
