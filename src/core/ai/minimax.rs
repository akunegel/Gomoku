use crate::core::GameState;
use crate::core::zobrist::Zobrist;
use crate::core::tt::{TranspositionTable, NodeType};
use std::time::{Instant, Duration};

use super::search::{get_candidates, move_heuristic};
use super::evaluation::evaluate_board;

pub fn find_best_move(state: &GameState, zobrist: &Zobrist) -> Option<(usize, usize)> {
    let start_time = Instant::now();
    let time_limit = Duration::from_millis(499); 
    let mut tt = TranspositionTable::new(256);

    if state.board.iter().flatten().all(|&cell| cell == 0) {
        return Some((9, 9));
    } 

    let depth = 10;
    let alpha = -200_000_000;
    let beta = 200_000_000;
    
    let result = search_at_depth(state, depth, alpha, beta, zobrist, &mut tt, &start_time, time_limit);

    result.map(|(m, _)| m).or_else(|| {
        for y in 0..19 {
            for x in 0..19 {
                if state.board[y][x] == 0 && state.can_place_piece(x, y).is_ok() {
                    return Some((x, y));
                }
            }
        }
        None
    })
}

fn search_at_depth(state: &GameState, depth: u32, mut alpha: i32, mut beta: i32, 
                    zobrist: &Zobrist, tt: &mut TranspositionTable, 
                    start_time: &Instant, time_limit: Duration) -> Option<((usize, usize), i32)> {
    
    let is_maximizing = state.current_player() == 1;
    let mut candidates = get_candidates(state);
    
    candidates.sort_by_cached_key(|&(x, y)| -move_heuristic(state, x, y));

    let mut best_move = None;
    let mut best_score = if is_maximizing { -200_000_000 } else { 200_000_000 };

    let max_branches = match depth {
        1..=3 => 12,
        4..=6 => 8,
        _     => 5,
    };

    for (x, y) in candidates.into_iter().take(max_branches) {
        if start_time.elapsed() >= time_limit { return None; }
        
        let mut next_state = state.clone();
        next_state.place_piece(x, y, zobrist);
        
        let score = alpha_beta(&next_state, depth - 1, alpha, beta, !is_maximizing, zobrist, tt, start_time, time_limit);
        
        if is_maximizing {
            if score > best_score { best_score = score; best_move = Some((x, y)); }
            alpha = alpha.max(score);
        } else {
            if score < best_score { best_score = score; best_move = Some((x, y)); }
            beta = beta.min(score);
        }
        if alpha >= beta { break; }
    }
    best_move.map(|m| (m, best_score))
}

fn alpha_beta(state: &GameState, depth: u32, mut alpha: i32, mut beta: i32, is_maximizing: bool, zobrist: &Zobrist, tt: &mut TranspositionTable, start_time: &Instant, time_limit: Duration) -> i32 {
    if start_time.elapsed() >= time_limit {
        return 0; 
    }

    if let Some(winner) = state.winner {
        return if winner == 1 { 100_000_000 + depth as i32 } else { -100_000_000 - depth as i32 };
    }
    if depth == 0 {
        return evaluate_board(state);
    }

    let mut tt_best_move = None;
    if let Some(entry) = tt.get(state.hash) {
        tt_best_move = entry.best_move;
        if entry.depth >= depth {
            match entry.node_type {
                NodeType::Exact => return entry.score,
                NodeType::LowerBound => alpha = alpha.max(entry.score),
                NodeType::UpperBound => beta = beta.min(entry.score),
            }
            if alpha >= beta { return entry.score; }
        }
    }

    let alpha_orig = alpha;
    let mut best_val = if is_maximizing { -200_000_000 } else { 200_000_000 };
    let mut current_best_move = None;

    let mut candidates = get_candidates(state);
    candidates.sort_by_cached_key(|&(x, y)| -move_heuristic(state, x, y));

    if let Some(m) = tt_best_move {
        if let Some(pos) = candidates.iter().position(|&x| x == m) {
            candidates.remove(pos);
            candidates.insert(0, m);
        }
    }

    let max_branches = if depth >= 6 { 4 } 
                       else if depth >= 3 { 6 } 
                       else { 4 };

    for (x, y) in candidates.into_iter().take(max_branches) {
        if state.can_place_piece(x, y).is_ok() {
            let mut next_state = state.clone();
            next_state.place_piece(x, y, zobrist);
            let eval = alpha_beta(&next_state, depth - 1, alpha, beta, !is_maximizing, zobrist, tt, start_time, time_limit);
            
            if is_maximizing {
                if eval > best_val { 
                    best_val = eval; 
                    current_best_move = Some((x, y)); 
                }
                alpha = alpha.max(eval);
            } else {
                if eval < best_val { 
                    best_val = eval; 
                    current_best_move = Some((x, y)); 
                }
                beta = beta.min(eval);
            }
            if beta <= alpha { break; }
        }
    }

    let node_type = if best_val <= alpha_orig { NodeType::UpperBound } 
                    else if best_val >= beta { NodeType::LowerBound } 
                    else { NodeType::Exact };
    tt.save(state.hash, depth, best_val, node_type, current_best_move);
    best_val
}