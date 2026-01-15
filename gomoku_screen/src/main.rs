// macroquadという名前のゲーム作成用道具箱を使う
use macroquad::prelude::*;

const BOARD_SIZE: usize = 19;
const CELL_SIZE: f32 = 30.0;
const OFFSET: f32 = 40.0;

struct Game {
    board: [[i32; 19]; 19],
    is_black_turn: bool,
    black_captures: i32,
    white_captures: i32,
}

impl Game {
    fn new() -> Self {
        Self {
            board: [[0; 19]; 19],
            is_black_turn: true,
            black_captures: 0,
            white_captures: 0,
        }
    }
}

// このプログラムはmacroquadというライブライを使って動かし、立ち上がったウィンドウでgomokuと表示する
#[macroquad::main("Gomoku")]
// 一回ループするごとに「画面を消すー＞線を引くー＞石を描くという動作を繰り返す
async fn main() {
    let mut game = Game::new();

    loop {
        clear_background(BEIGE);
        for i in 0..BOARD_SIZE {
            let pos = OFFSET + i as f32 * CELL_SIZE;
            draw_line(OFFSET, pos, OFFSET + 18.0 * CELL_SIZE, pos ,1.0, BLACK);
            draw_line(pos, OFFSET, pos, OFFSET + 18.0 * CELL_SIZE, 1.0, BLACK);
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let x = ((mx - OFFSET + 15.0) / CELL_SIZE) as i32;
            let y = ((my - OFFSET + 15.0) / CELL_SIZE) as i32;
            
            if x >= 0 && x < 19 && y >= 0 && y < 19 && game.board[y as usize][x as usize] == 0 {
                if game.is_black_turn {
                    game.board[y as usize][x as usize] = 1;
                } else {
                    game.board[y as usize][x as usize] = 2;
                }
                game.is_black_turn = !game.is_black_turn;
            }
        }
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if game.board[y][x] == 1 {
                    draw_circle(OFFSET + x as f32 * CELL_SIZE, OFFSET + y as f32 * CELL_SIZE, 13.0, BLACK);
                } else if game.board[y][x] == 2 {
                    draw_circle(OFFSET + x as f32 * CELL_SIZE, OFFSET + y as f32 * CELL_SIZE, 13.0, WHITE);
                    draw_circle_lines(OFFSET + x as f32 * CELL_SIZE, OFFSET + y as f32 * CELL_SIZE, 13.0, 1.0, BLACK);
                }
            }
        }
        let time_text = format!("Time: {:.2}s", get_time());
        draw_text(&time_text, 620.0, 50.0, 30.0, DARKGRAY);
        let capture_text = format!("Black Captures: {} | White Captures: {}", game.black_captures, game.white_captures);
        draw_text(&capture_text, 620.0, 100.0, 20.0, BLACK);

        next_frame().await
    }
}

// fn get_ai_move(current_board: &[[i32; 19]; 19]) -> (usize, usize) {
//     (10, 10)
// }