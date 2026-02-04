use super::capture::is_in_board;

pub fn check_win(board: &[[u8; 19]; 19], captures: &[u32; 2]) -> Option<u8> {
    if captures[0] >= 10 {
        return Some(1);
    }
    if captures[1] >= 10 {
        return Some(2);
    }

    for y in 0..19 {
        for x in 0..19 {
            let player = board[y][x];
            if player != 0 && is_five_aligned(board, y, x, player) {
                return Some(player);
            }
        }
    }
    None
}

fn is_five_aligned(board: &[[u8; 19]; 19], y: usize, x: usize, player: u8) -> bool {
    let directions = [(0, 1), (1, 0), (1, 1), (1, -1)];
    for (dy, dx) in directions {
        let mut count = 1;
        for i in 1..5 {
            let (ny, nx) = (y as i32 + dy * i, x as i32 + dx * i);
            if is_in_board(ny, nx) && board[ny as usize][nx as usize] == player {
                count += 1;
            } else { 
                break;
            }
        }
        for i in 1..5 {
            let (ny, nx) = (y as i32 - dy * i, x as i32 - dx * i);
            if is_in_board(ny, nx) && board[ny as usize][nx as usize] == player {
                count += 1;
            } else {
                break;
            }
        }
        if count >= 5 {
            return true;
        }
    }
    false
}