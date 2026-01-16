use macroquad::prelude::*;

mod game;
mod rules;

use game::Game;

const BOARD_SIZE: usize = 19;
const CELL_SIZE: f32 = 30.0;
const OFFSET: f32 = 40.0;

#[macroquad::main("Gomoku")]
async fn main() {
    let mut game = Game::new();

    loop {
        clear_background(BEIGE);
        draw_board();
        draw_stones(&game);
        draw_ui(&game);

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let x = ((mx - OFFSET + CELL_SIZE / 2.0) / CELL_SIZE) as i32;
            let y = ((my - OFFSET + CELL_SIZE / 2.0) / CELL_SIZE) as i32;
            
            if x >= 0 && x < 19 && y >= 0 && y < 19 && game.board[y as usize][x as usize] == 0 {
                game.place_stone(y as usize, x as usize);
            }
        }

        next_frame().await
    }
}

fn draw_board() {
    for i in 0..BOARD_SIZE {
        let pos = OFFSET + i as f32 * CELL_SIZE;
        draw_line(OFFSET, pos, OFFSET + (BOARD_SIZE - 1) as f32 * CELL_SIZE, pos, 1.0, BLACK);
        draw_line(pos, OFFSET, pos, OFFSET + (BOARD_SIZE - 1) as f32 * CELL_SIZE, 1.0, BLACK);
    }
}

fn draw_stones(game: &Game) {
    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            let center_x = OFFSET + x as f32 * CELL_SIZE;
            let center_y = OFFSET + y as f32 * CELL_SIZE;
            if game.board[y][x] == 1 {
                draw_circle(center_x, center_y, 13.0, BLACK);
            } else if game.board[y][x] == 2 {
                draw_circle(center_x, center_y, 13.0, WHITE);
                draw_circle_lines(center_x, center_y, 13.0, 1.0, BLACK);
            }
        }
    }
}

fn draw_ui(game: &Game) {
    let time_text = format!("Time: {:.2}s", get_time());
    draw_text(&time_text, 620.0, 50.0, 30.0, DARKGRAY);
    let capture_text = format!("Black: {} | White: {}", game.black_captures, game.white_captures);
    draw_text(&capture_text, 620.0, 100.0, 20.0, BLACK);
}

// fn get_ai_move(current_board: &[[i32; 19]; 19]) -> (usize, usize) {
//     (10, 10)
// }