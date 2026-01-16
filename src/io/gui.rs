use crate::core::game_state::GameState;
use super::interface::Interface;
use macroquad::prelude::*;
use std::pin::Pin;
use std::future::Future;

pub struct GuiInterface;

impl Interface for GuiInterface {
    fn render(&mut self, state: &GameState) {
        clear_background(BEIGE);
        const CELL_SIZE: f32 = 30.0;
        const OFFSET: f32 = 40.0;

        for i in 0..19 {
            let pos = OFFSET + i as f32 * CELL_SIZE;
            draw_line(OFFSET, pos, OFFSET + 18.0 * CELL_SIZE, pos, 1.0, BLACK);
            draw_line(pos, OFFSET, pos, OFFSET + 18.0 * CELL_SIZE, 1.0, BLACK);
        }

        for y in 0..19 {
            for x in 0..19 {
                let cell = state.board[y][x];
                if cell != 0 {
                    let cx = OFFSET + x as f32 * CELL_SIZE;
                    let cy = OFFSET + y as f32 * CELL_SIZE;
                    let color = if cell == 1 { BLACK } else { WHITE };
                    draw_circle(cx, cy, 13.0, color);
                    if cell == 2 { draw_circle_lines(cx, cy, 13.0, 1.0, BLACK); }
                }
            }
        }

        let player_name = if state.current_player() == 1 { "BLACK" } else { "WHITE" };
        draw_text(&format!("Turn: {}", player_name), 620.0, 50.0, 30.0, DARKGRAY);
    }

    fn get_move(&mut self, state: &GameState) -> Option<(usize, usize)> {
        if is_key_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let x = ((mx - 40.0 + 15.0) / 30.0).floor() as i32;
            let y = ((my - 40.0 + 15.0) / 30.0).floor() as i32;

            if x >= 0 && x < 19 && y >= 0 && y < 19 && state.board[y as usize][x as usize] == 0 {
                return Some((x as usize, y as usize));
            }
        }
        None
    }

    fn wait(&mut self) -> Pin<Box<dyn Future<Output = ()> + '_>> {
        Box::pin(async {
            next_frame().await;
        })
    }
}