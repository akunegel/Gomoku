use super::capture::is_in_board;

pub fn is_double_three(board: &[[u8; 19]; 19], y: usize, x: usize, player: u8) -> bool {
    let mut free_three_count = 0;
    let axes = [(0, 1), (1, 0), (1, 1), (1, -1)];

    let mut virtual_board = *board;
    virtual_board[y][x] = player;
    for (dy, dx) in axes {
        if is_free_three_on_axis(&virtual_board, y, x, dy, dx, player) {
            free_three_count += 1;
        }
    }
    free_three_count >= 2
}

fn is_free_three_on_axis(board: &[[u8; 19]; 19], y: usize, x: usize, dy: i32, dx: i32, player: u8) -> bool {
    let mut line = [0u8; 9];
    for i in -4..=4 {
        let ny = y as i32 + dy * i;
        let nx = x as i32 + dx * i;
        if is_in_board(ny, nx) {
            line[(i + 4) as usize] = board[ny as usize][nx as usize];
        } else {
            line[(i + 4) as usize] = 3;
        }
    }

    let p = player;
    let patterns = [
        vec![0, p, p, p, 0],
        vec![0, p, 0, p, p, 0],
        vec![0, p, p, 0, p, 0],
    ];

    for pat in patterns.iter() {
        if match_pattern(&line, pat) {
            return true;
        }
    }
    false
}

fn match_pattern(line: &[u8; 9], pattern: &Vec<u8>) -> bool {
    let pat_len = pattern.len();
    for start in 0..=(9 - pat_len) {
        let mut matched = true;
        let mut contains_center = false;

        for i in 0..pat_len {
            if line[start + i] != pattern[i] {
                matched = false;
                break;
            }
            if start + i == 4 {
                contains_center = true;
            }
        }

        if matched && contains_center {
            return true;
        }
    }
    false
}