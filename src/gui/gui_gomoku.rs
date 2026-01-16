use macroquad::prelude::*;
use crate::core::game_state::GameState;

const BOARD_SIZE: usize = 19;
const CELL_SIZE: f32 = 30.0;
const OFFSET: f32 = 40.0;
const BOARD_WIDTH: f32 = (BOARD_SIZE - 1) as f32 * CELL_SIZE;

pub fn start_game() {
    macroquad::Window::new("Gomoku", run_gui());
}

async fn run_gui() {
    let mut game = GameState::new();

    loop {
        clear_background(BEIGE);

        draw_board();
        draw_stones(&game);
        draw_ui(&game);

        if game.winner == 0 && is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            
            let x = ((mx - OFFSET + CELL_SIZE / 2.0) / CELL_SIZE).floor() as i32;
            let y = ((my - OFFSET + CELL_SIZE / 2.0) / CELL_SIZE).floor() as i32;
            
            if x >= 0 && x < 19 && y >= 0 && y < 19 {
                game.place_stone(y as usize, x as usize);
            }
        }
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await
    }
}

fn draw_board() {
    for i in 0..BOARD_SIZE {
        let pos = OFFSET + i as f32 * CELL_SIZE;
        draw_line(OFFSET, pos, OFFSET + BOARD_WIDTH, pos, 1.0, BLACK);
        draw_line(pos, OFFSET, pos, OFFSET + BOARD_WIDTH, 1.0, BLACK);
    }
}

fn draw_stones(game: &GameState) {
    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            let cx = OFFSET + x as f32 * CELL_SIZE;
            let cy = OFFSET + y as f32 * CELL_SIZE;
            
            match game.board[y][x] {
                1 => {
                    draw_circle(cx, cy, 13.0, BLACK);
                }
                2 => {
                    draw_circle(cx, cy, 13.0, WHITE);
                    draw_circle_lines(cx, cy, 13.0, 1.0, BLACK); // 縁取り
                }
                _ => {}
            }
        }
    }
}

fn draw_ui(game: &GameState) {
    let side_x = OFFSET + BOARD_WIDTH + 40.0;
    
    let turn_text = if game.is_black_turn { "Turn: BLACK" } else { "Turn: WHITE" };
    draw_text(turn_text, side_x, 50.0, 25.0, DARKGRAY);

    draw_text("Captures:", side_x, 100.0, 25.0, BLACK);
    draw_text(&format!("Black: {}", game.black_captures), side_x, 130.0, 20.0, BLACK);
    draw_text(&format!("White: {}", game.white_captures), side_x, 160.0, 20.0, BLACK);

}