use crate::core::GameState;

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

pub fn move_heuristic(state: &GameState, x: usize, y: usize) -> i32 {
    let p = state.current_player();
    let opp = if p == 1 { 2 } else { 1 };
    
    let mut score = 0;
    for &(dx, dy) in &[(1,0), (0,1), (1,1), (1,-1)] {
        let (cp, op) = check_pattern_at(state, x, y, dx, dy, p);
        let (co, oo) = check_pattern_at(state, x, y, dx, dy, opp);
        
        score += match (cp, op) {
            (5, _) => 10_00_000, (4, 2) => 1_000_000, (4, 1) => 500_000, (3, 2) => 200_000, _ => cp * 10,
        };
        score += match (co, oo) {
            (5, _) => 9_000_000,  (4, 2) => 2_000_000, (4, 1) => 1_500_000,  (3, 2) => 800_000, _ => co * 10,
        };
    }
    score + (10 - (9 - x as i32).abs() + 10 - (9 - y as i32).abs())
}

pub fn check_pattern_at(state: &GameState, x: usize, y: usize, dx: i32, dy: i32, p: u8) -> (i32, i32) {
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