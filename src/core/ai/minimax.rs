use crate::core::GameState;

pub fn find_best_move(state: &GameState) -> Option<(usize, usize)> {
    let mut best_score = i32::MIN;
    let mut best_move = None;
    let depth = 3; 

    let candidates = get_candidates(state);

    for (x, y) in candidates {
        if state.can_place_piece(x, y).is_ok() {
            let mut temp_state = state.clone();
            temp_state.place_piece(x, y);

            let score = -alpha_beta(&temp_state, depth - 1, i32::MIN + 1, i32::MAX - 1, false);
            
            if score > best_score {
                best_score = score;
                best_move = Some((x, y));
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
    if state.winner.is_some() || depth == 0 {
        return evaluate_board(state);
    }

    let candidates = get_candidates(state);
    
    if is_maximizing {
        let mut max_eval = i32::MIN;
        for (x, y) in candidates {
            if state.can_place_piece(x, y).is_ok() {
                let mut temp_state = state.clone();
                temp_state.place_piece(x, y); 
                let eval = alpha_beta(&temp_state, depth - 1, alpha, beta, false);
                max_eval = max_eval.max(eval);
                alpha = alpha.max(eval);
                if beta <= alpha { 
                    break; 
                }
            }
        }
        max_eval
    } else {
        let mut min_eval = i32::MAX;
        for (x, y) in candidates {
            let mut temp_state = state.clone();
            temp_state.place_piece(x, y);
            let eval = alpha_beta(&temp_state, depth - 1, alpha, beta, true);
            min_eval = min_eval.min(eval);
            beta = beta.min(eval);
            if beta <= alpha { break; }
        }
        min_eval
    }
}

fn evaluate_board(state: &GameState) -> i32 {
    let mut score = 0;

    score += (state.captures[0] as i32 - state.captures[1] as i32) * 1000;

    if let Some(winner) = state.winner {
        return if winner == 1 { 100000 } else { -100000 };
    }

    for y in 0..19 {
        for x in 0..19 {
            if state.board[y][x] == 1 {
                score += 10 - (x as i32 - 9).abs() + 10 - (y as i32 - 9).abs();
            } else if state.board[y][x] == 2 {
                score -= 10 - (x as i32 - 9).abs() + 10 - (y as i32 - 9).abs();
            }
        }
    }

    score
}