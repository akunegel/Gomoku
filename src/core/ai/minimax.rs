use crate::core::GameState;
use crate::core::zobrist::Zobrist;
use crate::core::tt::{TranspositionTable, NodeType};
use std::time::{Instant, Duration};

pub fn find_best_move(state: &GameState, zobrist: &Zobrist) -> Option<(usize, usize)> {
    let start_time = Instant::now();
    let time_limit = Duration::from_millis(480); 
    let mut tt = TranspositionTable::new(256);
    let mut best_move = None;

    if state.board.iter().flatten().all(|&cell| cell == 0) {
        return Some((9, 9));
    } 

    for depth in 1..=10 {
        if start_time.elapsed() >= time_limit { break; }
 
        if let Some((m, score)) = search_at_depth(state, depth, zobrist, &mut tt, &start_time, time_limit) {
            if start_time.elapsed() >= time_limit {
                break;
            }
            
            best_move = Some(m);
            println!("Depth {}: Best Move: {:?} Score: {}", depth, m, score);
            
            if score.abs() > 80_000_000 { break; }
        }
    }
    
    best_move.or_else(|| get_candidates(state).first().cloned())
}

fn search_at_depth(state: &GameState, depth: u32, zobrist: &Zobrist, tt: &mut TranspositionTable, start_time: &Instant, time_limit: Duration) -> Option<((usize, usize), i32)> {
    let mut alpha = -200_000_000;
    let mut beta = 200_000_000;
    let is_maximizing = state.current_player() == 1;

    let mut candidates = get_candidates(state);
    candidates.sort_by_cached_key(|&(x, y)| -move_heuristic(state, x, y));

    let mut best_move_at_depth = None;
    let mut best_score = if is_maximizing { -200_000_000 } else { 200_000_000 };

    let max_branches = 15;

    for (x, y) in candidates.into_iter().take(max_branches) {
        if start_time.elapsed() >= time_limit { break; }
        if state.can_place_piece(x, y).is_ok() {
            let mut next_state = state.clone();
            next_state.place_piece(x, y, zobrist);
            let score = alpha_beta(&next_state, depth - 1, alpha, beta, !is_maximizing, zobrist, tt, start_time, time_limit);

            if is_maximizing {
                if score > best_score {
                    best_score = score;
                    best_move_at_depth = Some((x, y));
                }
                alpha = alpha.max(score);
            } else {
                if score < best_score {
                    best_score = score;
                    best_move_at_depth = Some((x, y));
                }
                beta = beta.min(score);
            }
            if alpha >= beta { break; }
        }
    }
    best_move_at_depth.map(|m| (m, best_score))
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

    let max_branches = if depth >= 6 { 12 } 
                       else if depth >= 3 { 8 } 
                       else { 5 };

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

fn get_candidates(state: &GameState) -> Vec<(usize, usize)> {
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
                            visited[uy][ux] = true;
                            candidates.push((ux, uy));
                        }
                    }
                }
            }
        }
    }
    if candidates.is_empty() { candidates.push((9, 9)); }
    candidates
}

fn evaluate_board(state: &GameState) -> i32 {
    let p1_caps = state.captures[0] as i32;
    let p2_caps = state.captures[1] as i32;

    if p1_caps >= 5 { return 90_000_000; }
    if p2_caps >= 5 { return -90_000_000; }

    let mut score = 0;
    score += p1_caps * 150_000;
    score -= p2_caps * 150_000;

    let directions = [(1, 0), (0, 1), (1, 1), (1, -1)];
    for y in 0..19 {
        for x in 0..19 {
            let p = state.board[y][x];
            if p == 0 { continue; }

            for &(dx, dy) in &directions {
                let px = x as i32 - dx;
                let py = y as i32 - dy;
                if px >= 0 && px < 19 && py >= 0 && py < 19 {
                    if state.board[py as usize][px as usize] == p { continue; }
                }

                let (count, open) = get_line_info(state, x, y, dx, dy, p);
                let val = match (count, open) {
                    (5, _) => 10_000_000,
                    (4, 2) => 1_000_000,
                    (4, 1) => 200_000, 
                    (3, 2) => 100_000,
                    (3, 1) => 10_000,
                    (2, 2) => 1_000,
                    _ => 0,
                };
                if p == 1 { score += val; } else { score -= val; }
            }
        }
    }
    score
}

fn get_line_info(state: &GameState, x: usize, y: usize, dx: i32, dy: i32, p: u8) -> (i32, i32) {
    let mut count = 0;
    let mut open = 0;
    let bx = x as i32 - dx;
    let by = y as i32 - dy;
    if bx >= 0 && bx < 19 && by >= 0 && by < 19 && state.board[by as usize][bx as usize] == 0 {
        open += 1;
    }

    let mut cx = x as i32;
    let mut cy = y as i32;
    while cx >= 0 && cx < 19 && cy >= 0 && cy < 19 && state.board[cy as usize][cx as usize] == p {
        count += 1;
        cx += dx;
        cy += dy;
    }
    if cx >= 0 && cx < 19 && cy >= 0 && cy < 19 && state.board[cy as usize][cx as usize] == 0 {
        open += 1;
    }

    (count, open)
}

fn move_heuristic(state: &GameState, x: usize, y: usize) -> i32 {
    let p = state.current_player();
    let opp = if p == 1 { 2 } else { 1 };
    
    let mut score = 0;
    for &(dx, dy) in &[(1,0), (0,1), (1,1), (1,-1)] {
        let (cp, op) = check_pattern_at(state, x, y, dx, dy, p);
        let (co, oo) = check_pattern_at(state, x, y, dx, dy, opp);
        
        score += match (cp, op) {
            (5, _) => 100_000, (4, 2) => 50_000, (4, 1) => 10_000, (3, 2) => 5_000, _ => cp * 10,
        };
        score += match (co, oo) {
            (5, _) => 90_000,  (4, 2) => 40_000, (4, 1) => 8_000,  (3, 2) => 4_000, _ => co * 10,
        };
    }
    score + (10 - (9 - x as i32).abs() + 10 - (9 - y as i32).abs())
}

fn check_pattern_at(state: &GameState, x: usize, y: usize, dx: i32, dy: i32, p: u8) -> (i32, i32) {
    let mut count = 1;
    let mut open = 0;
    for &dir in &[1, -1] {
        for i in 1..5 {
            let nx = x as i32 + dx * i * dir;
            let ny = y as i32 + dy * i * dir;
            if nx < 0 || nx >= 19 || ny < 0 || ny >= 19 { break; }
            let cell = state.board[ny as usize][nx as usize];
            if cell == p { count += 1; }
            else if cell == 0 { open += 1; break; }
            else { break; }
        }
    }
    (count, open)
}