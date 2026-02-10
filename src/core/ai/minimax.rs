use crate::core::GameState;

pub fn find_best_move(state: &GameState) -> Option<(usize, usize)> {
    let mut best_score = i32::MIN;
    let mut best_move = None;
    let player = state.current_player();
    for y in 0..19 {
        for x in 0..19 {
            if state.can_place_piece(x, y).is_ok() {
                let score = evaluate_move(state, x, y, player);
                if score > best_score {
                    best_score = score;
                    best_move = Some((x, y ));
                }
            }
        }
    } 
    best_move
}

fn evaluate_move(state: &GameState, x: usize, y: usize, player: u8) -> i32 {
    let mut score = 0;
    let mut temp_state = state.clone();
    temp_state.place_piece(x, y);
    if temp_state.check_win() == Some(player) {
        return 10000;
    }
    score += (10 - (x as i32 - 9).abs()) + (10 - (y as i32 - 9).abs());
    score
}