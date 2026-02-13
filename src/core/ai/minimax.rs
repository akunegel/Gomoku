use crate::core::GameState;

pub fn find_best_move(state: &GameState) -> Option<(usize, usize)> {
    let current_p = state.current_player();
    let mut best_score = if current_p == 2 { i32::MAX } else { i32::MIN };
    let mut best_move = None;
    let depth = 4; 
    let is_maximizing = current_p == 1;
    let mut candidates = get_candidates(state);
    candidates.sort_by_cached_key(|&(x, y)| {
        let mut temp = state.clone();
        if temp.can_place_piece(x, y).is_ok() {
            temp.place_piece(x, y);
            let score = evaluate_board(&temp);
            if is_maximizing {
                -score
            } else {
                score
            }
        } else {
            0
        }
    }); 

    for (x, y) in candidates {
        if state.can_place_piece(x, y).is_ok() {
            let mut temp_state = state.clone();
            temp_state.place_piece(x, y);
            let next_is_maximizing = current_p == 2;

            let score = alpha_beta(&temp_state, depth - 1, i32::MIN + 1, i32::MAX - 1, next_is_maximizing);
            println!("Testing move ({}, {}), Score: {}", x, y, score);
            if current_p == 2 {
                if score < best_score {
                    best_score = score;
                    best_move = Some((x, y));
                }
            } else {
                if score > best_score {
                        best_score = score;
                        best_move = Some((x, y));
                }
            }
        }
    }
    best_move.or(Some((9, 9)))
}

fn get_candidates(state: &GameState) -> Vec<(usize, usize)> {
    let mut candidates = Vec::new();
    let mut visited = [[false; 19]; 19];

    for y in 0..19 {
        for x in 0..19 {
            if state.board[y][x] != 0 {
                for dy in -2..=2 {
                    for dx in -2..=2 {
                        let ny = y as i32 + dy;
                        let nx = x as i32 + dx;
                        if ny >= 0 && ny < 19 && nx >= 0 && nx < 19 {
                            let (ux, uy) = (nx as usize, ny as usize);
                            if state.board[uy][ux] == 0 && !visited[uy][ux] {
                                visited[uy][ux] = true;
                                candidates.push((ux, uy));
                            }
                        }
                    }
                }
            }
        }
    }
    candidates
}

fn alpha_beta(state: &GameState, depth: u32, mut alpha: i32, mut beta: i32, is_maximizing: bool) -> i32 {
    if let Some(winner) = state.winner {
        return if winner == 1 { 1000000 + depth as i32 } else { -1000000 - depth as i32 };
    }
    if depth == 0 {
        return evaluate_board(state);
    }

    let mut candidates = get_candidates(state);

    candidates.sort_by_cached_key(|&(x, y)| {
        let dx = x as i32 - 9;
        let dy = y as i32 - 9;
        dx * dx + dy * dy
    });
    let limited_candidates = candidates.into_iter().take(20);

    if is_maximizing {
        let mut max_eval = i32::MIN;
        for (x, y) in limited_candidates {
            if state.can_place_piece(x, y).is_ok() {
                let mut temp_state = state.clone();
                temp_state.place_piece(x, y); 
                let eval = alpha_beta(&temp_state, depth - 1, alpha, beta, false);
                max_eval = max_eval.max(eval);
                alpha = alpha.max(eval);
                if beta <= alpha { break; }
            }
        }
        max_eval
    } else {
        let mut min_eval = i32::MAX;
        for (x, y) in limited_candidates {
            if state.can_place_piece(x, y).is_ok() {
                let mut temp_state = state.clone();
                temp_state.place_piece(x, y);
                let eval = alpha_beta(&temp_state, depth - 1, alpha, beta, true);
                min_eval = min_eval.min(eval);
                beta = beta.min(eval);
                if beta <= alpha { break; }
            }
        }
        min_eval
    }
}

fn evaluate_board(state: &GameState) -> i32 {
    if let Some(winner) = state.winner {
        return if winner == 1 { 1000000 } else { -1000000 };
    }

    let mut score = 0;
    if let Some(pending) = state.five_aligned_winner {
        let pending_score = 800000;
        if pending == 1 {
            score += pending_score;
        } else {
            score -= pending_score;
        }
    }
    score += (state.captures[0] as i32).pow(2) * 5000;
    score -= (state.captures[1] as i32).pow(2) * 5000;

    for y in 0..19 {
        for x in 0..19 {
            let p = state.board[y][x];
            if p == 0 { continue; }
            
            for (dx, dy) in [(1,0), (0,1), (1,1), (1,-1)] {
                let prev_x = x as i32 - dx;
                let prev_y = y as i32 - dy;
                if prev_x >= 0 && prev_x < 19 && prev_y >= 0 && prev_y < 19 {
                    if state.board[prev_y as usize][prev_x as usize] == p { continue; }
                }
                
                let s = analyze_line(state, x, y, dx, dy, p);
                if p == 1 { score += s; } else { score -= s; }
            }
        }
    }
    score
}

fn analyze_line(state: &GameState, x: usize, y: usize, dx: i32, dy: i32, p: u8) -> i32 {
    let mut count = 0;
    let mut i = 0;
    while i < 5 {
        let nx = x as i32 + dx * i;
        let ny = y as i32 + dy * i;
        if nx < 0 || nx >= 19 || ny < 0 || ny >= 19 || state.board[ny as usize][nx as usize] != p {
            break;
        }
        count += 1;
        i += 1;
    }

    let mut open_ends = 0;
    let head_x = x as i32 - dx;
    let head_y = y as i32 - dy;
    if head_x >= 0 && head_x < 19 && head_y >= 0 && head_y < 19 {
        if state.board[head_y as usize][head_x as usize] == 0 { open_ends += 1; }
    }
    let tail_x = x as i32 + dx * count;
    let tail_y = y as i32 + dy * count;
    if tail_x >= 0 && tail_x < 19 && tail_y >= 0 && tail_y < 19 {
        if state.board[tail_y as usize][tail_x as usize] == 0 { open_ends += 1; }
    }

    match count {
        5 => 500000,
        4 => if open_ends >= 1 { 100000 } else { 0 },
        3 => if open_ends == 2 { 10000 } else { 1000 },
        2 => 100,
        _ => 0,
    }
}