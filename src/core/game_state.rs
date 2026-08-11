use super::rules::{capture, double_three};
use crate::core::zobrist::Zobrist;
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum GameMode {
    PVP,
    PVA,
    AVA,
}

#[derive(Clone)]
pub struct GameStateSnapshot {
    pub board: [[u8; 19]; 19],
    pub hash: u64,
    pub captures: [u32; 2],
    pub turn_count: u32,
    pub winner: Option<u8>,
    pub five_aligned_winner: Option<u8>,
}

#[derive(Clone, Copy)]
pub struct MoveInfo {
    pub number: u32,
    pub player: u8,
    pub x: usize,
    pub y: usize,
    pub captures: u32,
}

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
    pub history: Vec<GameStateSnapshot>,
    pub undos_since_last_move: u32,
}

impl Clone for GameState {
    fn clone(&self) -> Self {
        GameState {
            board: self.board,
            hash: self.hash,
            captures: self.captures,
            turn_count: self.turn_count,
            winner: self.winner,
            last_ai_time: self.last_ai_time,
            five_aligned_winner: self.five_aligned_winner,
            mode: self.mode,
            hint_move: self.hint_move,
            history: Vec::new(),
            undos_since_last_move: 0,
        }
    }
}

impl GameState {
    pub const MAX_UNDOS: u32 = 3;

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
            history: Vec::new(),
            undos_since_last_move: 0,
        }
    }

    pub fn push_history(&mut self) {
        self.history.push(GameStateSnapshot {
            board: self.board,
            hash: self.hash,
            captures: self.captures,
            turn_count: self.turn_count,
            winner: self.winner,
            five_aligned_winner: self.five_aligned_winner,
        });
        self.undos_since_last_move = 0;
    }

    pub fn undo_last_move(&mut self) -> bool {
        if self.history.is_empty() || self.undos_since_last_move >= Self::MAX_UNDOS {
            return false;
        }
        if let Some(prev) = self.history.pop() {
            self.board = prev.board;
            self.hash = prev.hash;
            self.captures = prev.captures;
            self.turn_count = prev.turn_count;
            self.winner = prev.winner;
            self.five_aligned_winner = prev.five_aligned_winner;
            self.hint_move = None;
            self.undos_since_last_move += 1;
            true
        } else {
            false
        }
    }

    pub fn move_history(&self) -> Vec<MoveInfo> {
        let mut moves = Vec::with_capacity(self.history.len());
        let mut prev: Option<&GameStateSnapshot> = None;
        for snap in &self.history {
            if let Some(before) = prev {
                moves.push(derive_move(&before.board, &snap.board, (moves.len() + 1) as u32));
            }
            prev = Some(snap);
        }
        if let Some(before) = prev {
            moves.push(derive_move(&before.board, &self.board, (moves.len() + 1) as u32));
        }
        moves
    }

    pub fn current_player(&self) -> u8 {
        ((self.turn_count % 2) + 1) as u8
    }

    pub fn recompute_hash(&mut self, zobrist: &Zobrist) {
        self.hash = compute_hash(&self.board, zobrist);
        for snap in &mut self.history {
            snap.hash = compute_hash(&snap.board, zobrist);
        }
    }

    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        use std::io::Write;
        let mut file = std::fs::File::create(path)?;
        writeln!(file, "gomoku_save")?;
        writeln!(file, "{}", mode_to_int(self.mode))?;
        writeln!(file, "{}", self.turn_count)?;
        writeln!(file, "{} {}", self.captures[0], self.captures[1])?;
        writeln!(file, "{}", self.winner.unwrap_or(0))?;
        writeln!(file, "{}", self.five_aligned_winner.unwrap_or(0))?;
        write_board(&mut file, &self.board)?;
        writeln!(file, "{}", self.history.len())?;
        for snap in &self.history {
            writeln!(file, "{} {}", snap.captures[0], snap.captures[1])?;
            writeln!(file, "{}", snap.turn_count)?;
            writeln!(file, "{}", snap.winner.unwrap_or(0))?;
            writeln!(file, "{}", snap.five_aligned_winner.unwrap_or(0))?;
            write_board(&mut file, &snap.board)?;
        }
        Ok(())
    }

    pub fn load_from_file(path: &str) -> std::io::Result<GameState> {
        let content = std::fs::read_to_string(path)?;
        let mut lines = content.lines();
        if next_line(&mut lines)? != "gomoku_save" {
            return Err(corrupted_file());
        }
        let mode = int_to_mode(read_u8(&mut lines)?).ok_or_else(corrupted_file)?;
        let turn_count = read_u32(&mut lines)?;
        let captures = read_captures(&mut lines)?;
        let winner = read_optional_u8(&mut lines)?;
        let five_aligned_winner = read_optional_u8(&mut lines)?;
        let board = read_board(&mut lines)?;
        let history_len = read_u32(&mut lines)?;
        let mut history = Vec::with_capacity(history_len as usize);
        for _ in 0..history_len {
            history.push(GameStateSnapshot {
                captures: read_captures(&mut lines)?,
                turn_count: read_u32(&mut lines)?,
                winner: read_optional_u8(&mut lines)?,
                five_aligned_winner: read_optional_u8(&mut lines)?,
                board: read_board(&mut lines)?,
                hash: 0,
            });
        }
        let mut state = GameState::new(mode);
        state.board = board;
        state.captures = captures;
        state.turn_count = turn_count;
        state.winner = winner;
        state.five_aligned_winner = five_aligned_winner;
        state.history = history;
        Ok(state)
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

fn derive_move(before: &[[u8; 19]; 19], after: &[[u8; 19]; 19], number: u32) -> MoveInfo {
    let mut placed = None;
    let mut captures = 0;
    for y in 0..19 {
        for x in 0..19 {
            if before[y][x] != after[y][x] {
                if after[y][x] != 0 {
                    placed = Some((x, y));
                } else {
                    captures += 1;
                }
            }
        }
    }
    let (x, y) = placed.expect("each move places a stone");
    MoveInfo {
        number,
        player: ((number - 1) % 2 + 1) as u8,
        x,
        y,
        captures,
    }
}

fn compute_hash(board: &[[u8; 19]; 19], zobrist: &Zobrist) -> u64 {
    let mut hash = 0;
    for (y, row) in board.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell != 0 {
                hash ^= zobrist.get_value(x, y, cell);
            }
        }
    }
    hash
}

fn mode_to_int(mode: GameMode) -> u8 {
    match mode {
        GameMode::PVP => 1,
        GameMode::PVA => 2,
        GameMode::AVA => 3,
    }
}

fn int_to_mode(value: u8) -> Option<GameMode> {
    match value {
        1 => Some(GameMode::PVP),
        2 => Some(GameMode::PVA),
        3 => Some(GameMode::AVA),
        _ => None,
    }
}

fn write_board(file: &mut std::fs::File, board: &[[u8; 19]; 19]) -> std::io::Result<()> {
    use std::io::Write;
    for row in board {
        for (i, cell) in row.iter().enumerate() {
            if i > 0 {
                write!(file, " ")?;
            }
            write!(file, "{}", cell)?;
        }
        writeln!(file)?;
    }
    Ok(())
}

fn next_line<'a>(lines: &mut std::str::Lines<'a>) -> std::io::Result<&'a str> {
    lines.next().ok_or_else(corrupted_file)
}

fn read_u8(lines: &mut std::str::Lines) -> std::io::Result<u8> {
    next_line(lines)?.trim().parse().map_err(|_| corrupted_file())
}

fn read_u32(lines: &mut std::str::Lines) -> std::io::Result<u32> {
    next_line(lines)?.trim().parse().map_err(|_| corrupted_file())
}

fn read_captures(lines: &mut std::str::Lines) -> std::io::Result<[u32; 2]> {
    let mut values = [0u32; 2];
    let line = next_line(lines)?;
    let mut tokens = line.split_whitespace();
    for value in values.iter_mut() {
        *value = tokens.next().and_then(|s| s.parse().ok()).ok_or_else(corrupted_file)?;
    }
    Ok(values)
}

fn read_optional_u8(lines: &mut std::str::Lines) -> std::io::Result<Option<u8>> {
    let value = read_u8(lines)?;
    Ok(if value == 0 { None } else { Some(value) })
}

fn read_board(lines: &mut std::str::Lines) -> std::io::Result<[[u8; 19]; 19]> {
    let mut board = [[0u8; 19]; 19];
    for row in board.iter_mut() {
        let line = next_line(lines)?;
        let mut tokens = line.split_whitespace();
        for cell in row.iter_mut() {
            *cell = tokens.next().and_then(|s| s.parse().ok()).ok_or_else(corrupted_file)?;
        }
    }
    Ok(board)
}

fn corrupted_file() -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, "corrupted save file")
}
