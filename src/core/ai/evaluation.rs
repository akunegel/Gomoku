use crate::core::GameState;

pub fn evaluate_board(state: &GameState) -> i32 {
    let p1_caps = state.captures[0] as i32;
    let p2_caps = state.captures[1] as i32;

    if p1_caps >= 5 { return 90_000_000; }
    if p2_caps >= 5 { return -90_000_000; }

    let mut score = 0;
    let mut offensive_capture = 400_000;
    let mut defensive_capture = 400_000;

    if p1_caps == 4 {
        offensive_capture = 5_000_000;
    }
    if p2_caps == 4 {
        defensive_capture = 5_000_000;
    }

    offensive_capture += (state.turn_count as i32) * 2_000;
    defensive_capture += (state.turn_count as i32) * 2_000;

    score += p1_caps * offensive_capture;
    score -= p2_caps * defensive_capture;

    let directions = [(1, 0), (0, 1), (1, 1), (1, -1)];
    for y in 0..19 {
        for x in 0..19 {
            let p = state.board[y][x];
            if p == 0 { continue; }

            for &(dx, dy) in &directions {
                if count_captures(state, x, y, dx, dy, p) {
                    let val = 100_000;
                    if p == 1 {
                        score += val;
                    } else {
                        score -= val;
                    }
                }
                if count_captures(state, x, y, -dx, -dy, p) {
                    let val = 100_000;
                    if p == 1 {
                        score += val;
                    } else {
                        score -= val;
                    }
                }
                let px = x as i32 - dx;
                let py = y as i32 - dy;
                if px >= 0 && px < 19 && py >= 0 && py < 19 {
                    if state.board[py as usize][px as usize] == p { continue; }
                }

                let (count, open) = get_line_info(state, x, y, dx, dy, p);
                let val = match (count, open) {
                    (5, _) => 50_000_000,
                    (4, 2) => 5_000_000,
                    (4, 1) => 50_000, 
                    (3, 2) => 250_000,
                    (3, 1) => 10_000,
                    (2, 2) => 5_000,
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

fn count_captures(state: &GameState, x: usize, y: usize, dx: i32, dy: i32, p: u8) -> bool {
    let opp = if p == 1 { 2 } else { 1 };

    let nx1 = x as i32 + dx;
    let ny1 = y as i32 + dy;
    let nx2 = x as i32 + 2 * dx;
    let ny2 = y as i32 + 2 * dy;
    let nx3 = x as i32 + 3 * dx;
    let ny3 = y as i32 + 3 * dy;

    if nx3 >= 0 && nx3 < 19 && ny3 >= 0 && ny3 < 19 {
        if state.board[ny1 as usize][nx1 as usize] == opp &&
              state.board[ny2 as usize][nx2 as usize] == opp &&
              state.board[ny3 as usize][nx3 as usize] == p {
                return true;
          }
    }
    false
}