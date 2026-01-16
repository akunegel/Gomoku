fn is_in_board(y: i32, x: i32) -> bool {
    x >= 0 && x < 19 && y >= 0 && y < 19
}

pub fn check_captures(board: &mut [[i32; 19]; 19], y: usize, x: usize) -> i32 {
    let mut captured_count = 0;
    let player = board[y][x];
    if player == 0 { return 0; }
    
    let opponent = if player == 1 { 2 } else { 1 };

    let directions = [
        (0, 1), (0, -1), (1, 0), (-1, 0),   // 上下左右
        (1, 1), (1, -1), (-1, 1), (-1, -1)  // 斜め
    ];
    
    for (dy, dx) in directions.iter() {
        let y1 = y as i32 + dy;
        let x1 = x as i32 + dx;
        let y2 = y as i32 + dy * 2;
        let x2 = x as i32 + dx * 2;
        let y3 = y as i32 + dy * 3;
        let x3 = x as i32 + dx * 3;

        if is_in_board(y3, x3) {
            let s1 = board[y1 as usize][x1 as usize];
            let s2 = board[y2 as usize][x2 as usize];
            let s3 = board[y3 as usize][x3 as usize];
            
            if s1 == opponent && s2 == opponent && s3 == player {
                board[y1 as usize][x1 as usize] = 0;
                board[y2 as usize][x2 as usize] = 0;
                captured_count += 2;
            }
        }
    }
    captured_count
}