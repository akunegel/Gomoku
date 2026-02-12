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
        if let Some(winner) = state.winner {
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.6));

            let text = if winner == 1 { "BLACK WINS!" } else { "WHITE WINS!" };
            draw_text(text, screen_width() / 2.0 - 120.0, screen_height() / 2.0, 50.0, WHITE);

            let btn_rect = Rect::new(screen_width() / 2.0 - 60.0, screen_height() / 2.0 + 40.0, 120.0, 40.0);
            draw_rectangle(btn_rect.x, btn_rect.y, btn_rect.w, btn_rect.h, RED);
            draw_text("CLOSE GAME", btn_rect.x + 10.0, btn_rect.y + 25.0, 20.0, WHITE);

            if is_mouse_button_pressed(MouseButton::Left) {
                let mouse_pos = mouse_position();
                if btn_rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1)) {
                    std::process::exit(0);
                }
            }
        }

        if let Some((hx, hy)) = state.hint_move {
            const H_CELL_SIZE: f32 = 30.0;
            const H_OFFSET: f32 = 40.0;
            
            let cx = H_OFFSET + hx as f32 * H_CELL_SIZE;
            let cy = H_OFFSET + hy as f32 * H_CELL_SIZE;

            draw_circle(cx, cy, H_CELL_SIZE * 0.4, Color::new(1.0, 1.0, 0.0, 0.6));
            draw_circle_lines(cx, cy, H_CELL_SIZE * 0.4, 2.0, YELLOW);
        }

        let player_name = if state.current_player() == 1 { "BLACK" } else { "WHITE" };
        draw_text(&format!("Turn: {}", player_name), 620.0, 50.0, 30.0, DARKGRAY);
        draw_text("Captures:", 620.0, 100.0, 25.0, DARKGRAY);
        draw_text(&format!("Black: {}", state.captures[0]), 620.0, 130.0, 25.0, BLACK);
        draw_text(&format!("White: {}", state.captures[1]), 620.0, 160.0, 25.0, WHITE);
        let timer_text = format!("AI Time: {:.4}s", state.last_ai_time);
        draw_text(&timer_text, 20.0, 40.0, 30.0, BLACK);
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

    fn is_key_pressed(&self, key: char) -> bool {
        match key {
            'H' => macroquad::prelude::is_key_pressed(KeyCode::H),
            _ => false,
        }
    }
}