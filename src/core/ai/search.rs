use crate::core::GameState;
use crate::core::rules::capture::is_in_board;
use crate::core::zobrist::Zobrist;

use super::evaluation::{pattern_value_after_place, CAPTURE_THREAT, VULNERABLE_PENALTY};

pub fn get_candidates(state: &GameState) -> Vec<(usize, usize)> {
    let mut candidates = Vec::with_capacity(40);
    let mut visited = [[false; 19]; 19];
    
    for y in 0..19 {
        for x in 0..19 {
            if state.board[y][x] != 0 {
                let min_dy = if y > 0 { -1 } else { 0 };
                let max_dy = if y < 18 { 1 } else { 0 };
                let min_dx = if x > 0 { -1 } else { 0 };
                let max_dx = if x < 18 { 1 } else { 0 };

                for dy in min_dy..=max_dy {
                    for dx in min_dx..=max_dx {
                        let uy = (y as i32 + dy) as usize;
                        let ux = (x as i32 + dx) as usize;
                        if state.board[uy][ux] == 0 && !visited[uy][ux] {
                            if state.can_place_piece(ux, uy).is_ok() {
                                visited[uy][ux] = true;
                                candidates.push((ux, uy));
                            }
                        }
                    }
                }
            }
        }
    }
    if candidates.is_empty() { 
        for y in 0..19 {
            for x in 0..19 {
                if state.board[y][x] == 0 && state.can_place_piece(x, y).is_ok() {
                    candidates.push((x, y));
                }
            }
        }
    }
    candidates
}

pub fn winning_move(state: &GameState, zobrist: &Zobrist) -> Option<(usize, usize)> {
    let p = state.current_player();
    for (x, y) in get_candidates(state) {
        let mut s = state.clone();
        s.place_piece(x, y, zobrist);
        if s.winner == Some(p) || s.has_five_aligned(p) {
            return Some((x, y));
        }
    }
    None
}

pub fn opponent_winning_move(state: &GameState, zobrist: &Zobrist) -> Option<(usize, usize)> {
    let opp = if state.current_player() == 1 { 2 } else { 1 };
    let mut view = state.clone();
    view.turn_count = if opp == 1 { 0 } else { 1 };
    winning_move(&view, zobrist)
}

pub fn move_heuristic(state: &GameState, x: usize, y: usize) -> i32 {
    let p = state.current_player();
    let opp = if p == 1 { 2 } else { 1 };
    let board = &state.board;

    let mut score: i64 = 0;

    let mut captured = 0;
    for &(dx, dy) in &[(1, 0), (0, 1), (1, 1), (1, -1)] {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        let fx = x as i32 + 2 * dx;
        let fy = y as i32 + 2 * dy;
        let ax = x as i32 + 3 * dx;
        let ay = y as i32 + 3 * dy;
        if is_in_board(ay, ax)
            && board[ny as usize][nx as usize] == opp
            && board[fy as usize][fx as usize] == opp
            && board[ay as usize][ax as usize] == p
        {
            captured += 2;
        }
        let bn = x as i32 - dx;
        let bny = y as i32 - dy;
        let bf = x as i32 - 2 * dx;
        let bfy = y as i32 - 2 * dy;
        let ba = x as i32 - 3 * dx;
        let bay = y as i32 - 3 * dy;
        if is_in_board(bay, ba)
            && board[bny as usize][bn as usize] == opp
            && board[bfy as usize][bf as usize] == opp
            && board[bay as usize][ba as usize] == p
        {
            captured += 2;
        }
    }
    if captured > 0 {
        let after = state.captures[(p - 1) as usize] as i32 + captured;
        if after >= 10 {
            score += 90_000_000;
        }
        score += (captured as i64) * (CAPTURE_THREAT as i64 / 2);
    }

    for &(dx, dy) in &[(1, 0), (0, 1), (1, 1), (1, -1)] {
        score += pattern_value_after_place(board, x, y, dx, dy, p) as i64;
        score += pattern_value_after_place(board, x, y, dx, dy, opp) as i64;
    }

    for &(dx, dy) in &[(1, 0), (0, 1), (1, 1), (1, -1)] {
        for &sign in &[1i32, -1i32] {
            let nx = x as i32 + sign * dx;
            let ny = y as i32 + sign * dy;
            if is_in_board(ny, nx) && board[ny as usize][nx as usize] == p {
                let bx = x as i32 - sign * dx;
                let by = y as i32 - sign * dy;
                let ax = nx + sign * dx;
                let ay = ny + sign * dy;
                let b_opp = is_in_board(by, bx) && board[by as usize][bx as usize] == opp;
                let b_empty = is_in_board(by, bx) && board[by as usize][bx as usize] == 0;
                let a_opp = is_in_board(ay, ax) && board[ay as usize][ax as usize] == opp;
                let a_empty = is_in_board(ay, ax) && board[ay as usize][ax as usize] == 0;
                if (b_opp && a_empty) || (b_empty && a_opp) {
                    score -= VULNERABLE_PENALTY as i64;
                }
            }
        }
    }

    score += ((10 - (9 - x as i32).abs()) + (10 - (9 - y as i32).abs())) as i64 * 100;

    score.clamp(i32::MIN as i64, i32::MAX as i64) as i32
}
