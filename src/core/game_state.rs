use super::rules::{capture, double_three};
use crate::core::zobrist::Zobrist;
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum GameMode {
    PVP,
    PVA,
}

#[derive(Clone)]
pub struct GameState {
    pub board: [[u8; 19]; 19],
    pub hash: u64,
    pub captures: [u32; 2],
    pub turn_count: u32,
    pub winner: Option<u8>,
    pub last_ai_time: f64,
    pub five_aligned_winner: Option<u8>,
    pub mode: GameMode,
    pub hint_move: Option<(usize, usize)>,
}

impl GameState {
    pub fn new(mode: GameMode) -> Self {
        GameState {
            board: [[0; 19]; 19],
            hash: 0,
            captures: [0, 0],
            turn_count: 0,
            winner: None,
            last_ai_time: 0.0,
            five_aligned_winner: None,
            mode,
            hint_move: None,
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

    pub fn place_piece(&mut self, x: usize, y: usize, zobrist: &Zobrist) {
        if self.winner.is_some() { return; }

        let p_current = self.current_player();
        let opponent = if p_current == 1 { 2 } else { 1 };

        self.board[y][x] = p_current;
        self.update_hash(x, y, p_current, zobrist);
        let captured_coords = capture::apply_captures(&mut self.board, y, x);
        
        for (cx, cy) in &captured_coords {
            self.update_hash(*cx, *cy, opponent, zobrist); 
        }

        self.captures[(p_current - 1) as usize] += captured_coords.len() as u32;
        if self.captures[(p_current - 1) as usize] >= 10 {
            self.winner = Some(p_current);
            return;
        }

        if let Some(pending) = self.five_aligned_winner {
            if pending == opponent {
                if self.has_five_aligned(opponent) {
                    self.winner = Some(opponent);
                    return;
                } else {
                    // println!("The five-in-a-row was broken! Game continues.");
                    self.five_aligned_winner = None;
                }
            }
        }

        if self.has_five_aligned(p_current) {
            self.five_aligned_winner = Some(p_current);
            // println!("Five in a row! Next player, try to break it!");
        }

        self.turn_count += 1;
    } 


    pub fn has_five_aligned(&self, target_player: u8) -> bool {
        let directions = [(0, 1), (1, 0), (1, 1), (1, -1)];

        for y in 0..19 {
            for x in 0..19 {
                let player = self.board[y][x];
                if player != target_player { 
                    continue;
                }
                for (dy, dx) in directions {
                    let mut count = 1;
                    for i in 1..5 {
                        let ny = y as i32 + dy * i;
                        let nx = x as i32 + dx * i;
                        
                        if ny >= 0 && ny < 19 && nx >= 0 && nx < 19 
                           && self.board[ny as usize][nx as usize] == player {
                            count += 1;
                        } else {
                            break;
                        }
                    }
                    if count >= 5 {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn update_hash(&mut self, x: usize, y: usize, player: u8, zobrist: &Zobrist) {
        if player == 0 {
            return;
        }
        self.hash ^= zobrist.get_value(x, y, player);
    }
}
