use crate::core::GameState;
use crate::core::rules::capture::is_in_board;

pub const CAPTURE_THREAT: i32 = 6_000_000;
pub const VULNERABLE_PENALTY: i32 = 4_000_000;

const FIVE: i32 = 99_000_000;
const OPEN_FOUR: i32 = 55_000_000;
const FOUR: i32 = 14_000_000;
const OPEN_THREE: i32 = 3_000_000;
const THREE: i32 = 600_000;
const OPEN_TWO: i32 = 40_000;
const TWO: i32 = 15_000;

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (1, 1), (1, -1)];

pub fn evaluate_board(state: &GameState) -> i32 {
    if state.captures[0] >= 10 {
        return FIVE;
    }
    if state.captures[1] >= 10 {
        return -FIVE;
    }

    let mut score: i64 = 0;
    score += capture_progress(state.captures[0]) as i64;
    score -= capture_progress(state.captures[1]) as i64;

    score += pattern_score(state, 1) as i64;
    score -= pattern_score(state, 2) as i64;

    score += count_capture_threats(&state.board, 1) as i64 * CAPTURE_THREAT as i64;
    score -= count_capture_threats(&state.board, 2) as i64 * CAPTURE_THREAT as i64;

    score.clamp(i32::MIN as i64, i32::MAX as i64) as i32
}

fn capture_progress(caps: u32) -> i32 {
    let c = caps.min(10) as i32;
    (c * (1_500_000 + c * 400_000)).min(FIVE)
}

fn pattern_score(state: &GameState, p: u8) -> i32 {
    let board = &state.board;
    let mut total: i64 = 0;
    for y in 0..19 {
        for x in 0..19 {
            if board[y][x] != p {
                continue;
            }
            total += ((10 - (9 - x as i32).abs()) + (10 - (9 - y as i32).abs())) as i64 * 30;
            for &(dx, dy) in &DIRECTIONS {
                let bx = x as i32 - dx;
                let by = y as i32 - dy;
                if is_in_board(by, bx) && board[by as usize][bx as usize] == p {
                    continue;
                }
                total += classify_run(board, x, y, dx, dy, p) as i64;
            }
        }
    }
    total.clamp(i32::MIN as i64, i32::MAX as i64) as i32
}

fn classify_run(board: &[[u8; 19]; 19], x: usize, y: usize, dx: i32, dy: i32, p: u8) -> i32 {
    let mut line = [0u8; 11];
    let mut len = 0;
    let px = x as i32 - dx;
    let py = y as i32 - dy;
    line[len] = if is_in_board(py, px) {
        code_of(board[py as usize][px as usize], p)
    } else {
        2
    };
    len += 1;

    let mut cx = x as i32;
    let mut cy = y as i32;
    for _ in 0..9 {
        let code = if is_in_board(cy, cx) {
            let c = code_of(board[cy as usize][cx as usize], p);
            line[len] = c;
            len += 1;
            c
        } else {
            line[len] = 2;
            len += 1;
            2
        };
        if code == 2 {
            break;
        }
        cx += dx;
        cy += dy;
    }
    classify_line(&line[..len])
}

pub fn pattern_value_after_place(
    board: &[[u8; 19]; 19],
    x: usize,
    y: usize,
    dx: i32,
    dy: i32,
    p: u8,
) -> i32 {
    let mut sx = x as i32;
    let mut sy = y as i32;
    loop {
        let px = sx - dx;
        let py = sy - dy;
        if is_in_board(py, px) && board[py as usize][px as usize] == p {
            sx = px;
            sy = py;
        } else {
            break;
        }
    }

    let mut line = [0u8; 11];
    let mut len = 0;
    let px = sx - dx;
    let py = sy - dy;
    line[len] = if is_in_board(py, px) {
        code_of(board[py as usize][px as usize], p)
    } else {
        2
    };
    len += 1;

    let mut cx = sx;
    let mut cy = sy;
    for _ in 0..9 {
        let cell = if cx == x as i32 && cy == y as i32 {
            p
        } else if is_in_board(cy, cx) {
            board[cy as usize][cx as usize]
        } else {
            3
        };
        let code = if cell == p {
            1
        } else if cell == 0 {
            0
        } else {
            2
        };
        line[len] = code;
        len += 1;
        if code == 2 {
            break;
        }
        cx += dx;
        cy += dy;
    }
    classify_line(&line[..len])
}

fn classify_line(line: &[u8]) -> i32 {
    let n = line.len();
    if n < 2 || line[1] != 1 {
        return 0;
    }

    let mut run = 0;
    while 1 + run < n && line[1 + run] == 1 {
        run += 1;
    }
    if run >= 5 {
        return FIVE;
    }

    let pre = line[0];
    let post = if 1 + run < n { line[1 + run] } else { 2 };
    let mut best: i32 = 0;

    if run == 4 {
        best = best.max(match (pre, post) {
            (0, 0) => OPEN_FOUR,
            (2, 2) => THREE,
            _ => FOUR,
        });
    } else if run == 3 {
        best = best.max(match (pre, post) {
            (0, 0) => OPEN_THREE,
            (2, 2) => TWO,
            _ => THREE,
        });
    } else if run == 2 {
        best = best.max(if pre == 0 && post == 0 {
            OPEN_TWO
        } else {
            TWO
        });
    }

    for z in 1..n {
        if line[z] != 0 {
            continue;
        }
        let mut left = 0;
        let mut i = z;
        while i > 0 && line[i - 1] == 1 {
            left += 1;
            i -= 1;
        }
        let mut right = 0;
        let mut j = z + 1;
        while j < n && line[j] == 1 {
            right += 1;
            j += 1;
        }
        let total = left + 1 + right;
        if total >= 5 {
            best = best.max(FOUR);
        } else if total == 4 {
            let start = z - left;
            let end = z + right;
            let left_open = start > 0 && line[start - 1] == 0;
            let right_open = end + 1 < n && line[end + 1] == 0;
            best = best.max(if left_open && right_open {
                OPEN_THREE
            } else {
                THREE
            });
        }
    }
    best
}

fn count_capture_threats(board: &[[u8; 19]; 19], p: u8) -> i32 {
    let opp = if p == 1 { 2 } else { 1 };
    let mut count = 0;
    for y in 0..19 {
        for x in 0..19 {
            if board[y][x] != p {
                continue;
            }
            for &(dx, dy) in &DIRECTIONS {
                for &s in &[1i32, -1i32] {
                    let x1 = x as i32 + s * dx;
                    let y1 = y as i32 + s * dy;
                    let x2 = x as i32 + 2 * s * dx;
                    let y2 = y as i32 + 2 * s * dy;
                    let x3 = x as i32 + 3 * s * dx;
                    let y3 = y as i32 + 3 * s * dy;
                    if is_in_board(y3, x3)
                        && board[y1 as usize][x1 as usize] == opp
                        && board[y2 as usize][x2 as usize] == opp
                        && board[y3 as usize][x3 as usize] == 0
                    {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

#[inline]
fn code_of(cell: u8, p: u8) -> u8 {
    if cell == p {
        1
    } else if cell == 0 {
        0
    } else {
        2
    }
}
