use crate::core::game_state::GameState;
use super::interface::{Interface, PlayerAction};
use crate::core::ai::{RootCandidate, SearchProgress, SearchReport};
use macroquad::prelude::*;
use std::pin::Pin;
use std::future::Future;

const CELL_SIZE: f32 = 30.0;
const OFFSET: f32 = 40.0;
const PANEL_X: f32 = 620.0;

pub struct GuiInterface {
    visualizer: bool,
    search_start: Option<std::time::Instant>,
}

impl GuiInterface {
    pub fn new(visualizer: bool) -> Self {
        GuiInterface {
            visualizer,
            search_start: None,
        }
    }

    fn track_search_start(&mut self, state: &GameState) {
        if state.search_progress.is_some() {
            if self.search_start.is_none() {
                self.search_start = Some(std::time::Instant::now());
            }
        } else {
            self.search_start = None;
        }
    }
}

impl Interface for GuiInterface {
    fn render(&mut self, state: &GameState) {
        clear_background(BEIGE);
        self.track_search_start(state);

        draw_board_grid();
        draw_stones(state);

        
        draw_text(&format!("AI Time: {:.4}s", state.last_ai_time), 20.0, 40.0, 30.0, BLACK);
        
        if self.visualizer {
            draw_turn_captures(state);
            draw_board_markers(state);
            draw_analysis(state, self.search_start);
            draw_moves_list(state, 1000.0, 50.0);
        } else {
            draw_hint(state);
            draw_turn_captures(state);
            draw_moves_list(state, 620.0, 210.0);
        }
        if let Some(winner) = state.winner {
            draw_winner_overlay(winner);
        }
    }

    fn get_action(&mut self, state: &GameState) -> Option<PlayerAction> {
        if is_key_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let x = ((mx - 40.0 + 15.0) / 30.0).floor() as i32;
            let y = ((my - 40.0 + 15.0) / 30.0).floor() as i32;

            if x >= 0 && x < 19 && y >= 0 && y < 19 && state.board[y as usize][x as usize] == 0 {
                return Some(PlayerAction::Place((x as usize, y as usize)));
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
            'Z' => macroquad::prelude::is_key_pressed(KeyCode::Z),
            'S' => macroquad::prelude::is_key_pressed(KeyCode::S),
            _ => false,
        }
    }

    fn visualizer_enabled(&self) -> bool {
        self.visualizer
    }

    fn get_save_path(&mut self) -> Pin<Box<dyn Future<Output = Option<String>> + '_>> {
        Box::pin(async {
            clear_input_queue();
            let mut path = String::new();
            loop {
                clear_background(BEIGE);
                draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.6));
                draw_text("Enter save path:", 200.0, 250.0, 30.0, WHITE);
                draw_rectangle(200.0, 280.0, 400.0, 40.0, WHITE);
                draw_text(&path, 210.0, 310.0, 30.0, BLACK);
                if is_key_pressed(KeyCode::Escape) {
                    return None;
                }
                if is_key_pressed(KeyCode::Enter) && !path.is_empty() {
                    return Some(path);
                }
                if is_key_pressed(KeyCode::Backspace) {
                    path.pop();
                }
                while let Some(c) = get_char_pressed() {
                    path.push(c);
                }
                next_frame().await;
            }
        })
    }

    fn wait_for_continue<'a>(&'a mut self, state: &'a GameState) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
        Box::pin(async move {
            loop {
                self.render(state);
                draw_text("Press SPACE or click to continue...", PANEL_X, 670.0, 20.0, DARKGRAY);
                next_frame().await;
                if is_key_pressed(KeyCode::Space)
                    || is_key_pressed(KeyCode::Enter)
                    || is_mouse_button_pressed(MouseButton::Left)
                {
                    break;
                }
            }
        })
    }
}

fn draw_board_grid() {
    for i in 0..19 {
        let pos = OFFSET + i as f32 * CELL_SIZE;
        draw_line(OFFSET, pos, OFFSET + 18.0 * CELL_SIZE, pos, 1.0, BLACK);
        draw_line(pos, OFFSET, pos, OFFSET + 18.0 * CELL_SIZE, 1.0, BLACK);
    }
}

fn draw_stones(state: &GameState) {
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
}

fn draw_winner_overlay(winner: u8) {
    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.05, 0.05, 0.08, 0.9));

    let panel_w = 420.0;
    let panel_h = 200.0;
    let panel_x = (screen_width() - panel_w) / 2.0;
    let panel_y = (screen_height() - panel_h) / 2.0;

    draw_rectangle(panel_x, panel_y, panel_w, panel_h, Color::new(0.15, 0.15, 0.2, 1.0));
    draw_rectangle_lines(panel_x, panel_y, panel_w, panel_h, 2.0, WHITE);

    let text = if winner == 1 { "BLACK WINS!" } else { "WHITE WINS!" };
    let text_color = if winner == 1 { WHITE } else { WHITE };
    let font_size = 38.0;
    
    let text_dims = measure_text(text, None, font_size as u16, 1.0);
    let text_x = panel_x + (panel_w - text_dims.width) / 2.0;
    let text_y = panel_y + 75.0;
    
    draw_text(text, text_x, text_y, font_size, text_color);

    let btn_w = 160.0;
    let btn_h = 40.0;
    let btn_x = panel_x + (panel_w - btn_w) / 2.0;
    let btn_y = panel_y + 120.0;
    
    draw_rectangle(btn_x, btn_y, btn_w, btn_h, Color::new(0.8, 0.2, 0.2, 1.0));
    draw_rectangle_lines(btn_x, btn_y, btn_w, btn_h, 1.0, WHITE);
    
    let btn_text = "CLOSE GAME";
    let btn_text_dims = measure_text(btn_text, None, 18, 1.0);
    let btn_text_x = btn_x + (btn_w - btn_text_dims.width) / 2.0;
    let btn_text_y = btn_y + 25.0;
    
    draw_text(btn_text, btn_text_x, btn_text_y, 18.0, WHITE);

    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_pos = mouse_position();
        if Rect::new(btn_x, btn_y, btn_w, btn_h).contains(Vec2::new(mouse_pos.0, mouse_pos.1)) {
            std::process::exit(0);
        }
    }
}

fn draw_hint(state: &GameState) {
    if let Some((hx, hy)) = state.hint_move {
        let cx = OFFSET + hx as f32 * CELL_SIZE;
        let cy = OFFSET + hy as f32 * CELL_SIZE;

        draw_circle(cx, cy, CELL_SIZE * 0.4, Color::new(1.0, 1.0, 0.0, 0.6));
        draw_circle_lines(cx, cy, CELL_SIZE * 0.4, 2.0, YELLOW);
    }
}

fn draw_turn_captures(state: &GameState) {
    let player_name = if state.current_player() == 1 { "BLACK" } else { "WHITE" };
    draw_text(&format!("Turn: {}", player_name), 620.0, 50.0, 30.0, DARKGRAY);
    draw_text("Captures:", 620.0, 100.0, 25.0, DARKGRAY);
    draw_text(&format!("Black: {}", state.captures[0]), 620.0, 130.0, 25.0, BLACK);
    draw_text(&format!("White: {}", state.captures[1]), 620.0, 160.0, 25.0, WHITE);
}

fn draw_moves_list(state: &GameState, x: f32, y: f32) {
    draw_text("Moves:", x, y, 25.0, DARKGRAY);
    const MAX_ROWS: usize = 18;
    for (i, m) in state.move_history().iter().enumerate() {
        let player = if m.player == 1 { "B" } else { "W" };
        let text = format!("{}. {}({},{})", m.number, player, m.x, m.y);
        let column = (i / MAX_ROWS) as f32;
        let row = (i % MAX_ROWS) as f32;
        draw_text(&text, x + column * 95.0, y + 30.0 + row * 18.0, 16.0, DARKGRAY);
    }
}

fn draw_board_markers(state: &GameState) {
    let live = state.search_progress.as_ref().map(|p| p.lock().unwrap().clone());
    let report = state.last_search.clone();

    let rows: Vec<(usize, usize, usize, bool)> = if let Some(p) = live {
        p.root_results
            .iter()
            .enumerate()
            .map(|(i, rc)| (i + 1, rc.x, rc.y, p.best_move_so_far == Some((rc.x, rc.y))))
            .collect()
    } else if let Some(r) = report {
        r.candidates
            .iter()
            .enumerate()
            .map(|(i, rc)| (i + 1, rc.x, rc.y, r.best_move == Some((rc.x, rc.y))))
            .collect()
    } else {
        Vec::new()
    };

    for (rank, x, y, is_best) in rows {
        if state.board[y][x] != 0 {
            continue;
        }
        let cx = OFFSET + x as f32 * CELL_SIZE;
        let cy = OFFSET + y as f32 * CELL_SIZE;
        if is_best {
            draw_circle_lines(cx, cy, CELL_SIZE * 0.45, 3.0, YELLOW);
        }
        let text = rank.to_string();
        let m = measure_text(&text, None, 14, 1.0);
        draw_text(&text, cx - m.width / 2.0, cy + m.height / 2.0, 14.0, BLUE);
    }
}

fn draw_analysis(state: &GameState, search_start: Option<std::time::Instant>) {
    let live = state.search_progress.as_ref().map(|p| p.lock().unwrap().clone());
    let report = state.last_search.clone();

    let mut y = 190.0;
    draw_text("SOLVER ANALYSIS", PANEL_X, y, 24.0, DARKGRAY);
    y += 30.0;

    if let Some(p) = live {
        draw_searching_panel(&p, search_start, &mut y);
    } else if let Some(r) = report {
        draw_report_panel(&r, &mut y);
    } else {
        draw_text("no analysis yet", PANEL_X, y, 16.0, LIGHTGRAY);
    }
}

fn draw_searching_panel(p: &SearchProgress, search_start: Option<std::time::Instant>, y: &mut f32) {
    draw_text("SEARCHING...", PANEL_X, *y, 20.0, ORANGE);
    *y += 26.0;

    let elapsed = search_start.map(|s| s.elapsed().as_millis()).unwrap_or(0);
    draw_text(&format!("elapsed: {} ms", elapsed), PANEL_X, *y, 16.0, DARKGRAY);
    *y += 20.0;

    draw_text(
        &format!("nodes: {}   tt hits: {}", format_thousands(p.nodes as usize), format_thousands(p.tt_hits as usize)),
        PANEL_X, *y, 16.0, DARKGRAY,
    );
    *y += 20.0;

    match p.current_candidate {
        Some((x, yy)) => { draw_text(&format!("analyzing: ({}, {})", x, yy), PANEL_X, *y, 16.0, BLACK); }
        None => { draw_text("analyzing: -", PANEL_X, *y, 16.0, BLACK); }
    }
    *y += 20.0;

    if let Some((bx, by)) = p.best_move_so_far {
        draw_text(&format!("best so far: ({}, {})  {}", bx, by, format_score(p.best_score_so_far)), PANEL_X, *y, 16.0, DARKGREEN);
        *y += 20.0;
    }

    let total = p.total_candidates.max(1);
    let frac = (p.candidates_done as f32 / total as f32).clamp(0.0, 1.0);
    draw_rectangle(PANEL_X, *y, 200.0, 10.0, LIGHTGRAY);
    draw_rectangle(PANEL_X, *y, 200.0 * frac, 10.0, BLUE);
    draw_text(&format!("{}/{} branches", p.candidates_done, p.total_candidates), PANEL_X + 210.0, *y + 10.0, 14.0, DARKGRAY);
    *y += 30.0;

    draw_candidate_table(&p.root_results, p.best_move_so_far, y);
}

fn draw_report_panel(r: &SearchReport, y: &mut f32) {
    if let Some((bx, by)) = r.best_move {
        let player = if r.player == 1 { "BLACK" } else { "WHITE" };
        draw_text(&format!("best: ({}, {}) for {}", bx, by, player), PANEL_X, *y, 20.0, BLACK);
        *y += 26.0;
        draw_text(&format!("score: {}", format_score(r.best_score)), PANEL_X, *y, 20.0, BLACK);
        *y += 26.0;
    }

    draw_text(
        &format!("time: {} ms   depth: {}", r.elapsed_ms, r.depth),
        PANEL_X, *y, 16.0, DARKGRAY,
    );
    *y += 20.0;

    draw_text(
        &format!("nodes: {}   tt hits: {}", format_thousands(r.nodes as usize), format_thousands(r.tt_hits as usize)),
        PANEL_X, *y, 16.0, DARKGRAY,
    );
    *y += 20.0;

    if r.timed_out {
        draw_text("! search hit time limit", PANEL_X, *y, 14.0, ORANGE);
        *y += 18.0;
    }

    *y += 8.0;
    draw_candidate_table(&r.candidates, r.best_move, y);
}

fn draw_candidate_table(cands: &[RootCandidate], best: Option<(usize, usize)>, y: &mut f32) {
    if cands.is_empty() {
        draw_text("no candidates", PANEL_X, *y, 14.0, LIGHTGRAY);
        *y += 18.0;
        return;
    }

    draw_text("#", PANEL_X, *y, 14.0, DARKGRAY);
    draw_text("move", 650.0, *y, 14.0, DARKGRAY);
    draw_text("heur", 720.0, *y, 14.0, DARKGRAY);
    draw_text("score", 800.0, *y, 14.0, DARKGRAY);
    draw_text("flag", 950.0, *y, 14.0, DARKGRAY);
    *y += 18.0;

    let max_abs = cands.iter().map(|c| c.score.abs()).max().unwrap_or(1).max(1);
    for (i, c) in cands.iter().enumerate() {
        if *y > 660.0 { break; }
        let is_best = best == Some((c.x, c.y));
        if is_best {
            draw_rectangle(PANEL_X - 4.0, *y - 14.0, 320.0, 18.0, Color::new(1.0, 0.85, 0.2, 0.5));
        }
        let color = if c.timed_out {
            LIGHTGRAY
        } else if c.score > 0 {
            DARKGREEN
        } else if c.score < 0 {
            MAROON
        } else {
            DARKGRAY
        };

        draw_text(&format!("{}", i + 1), PANEL_X, *y, 14.0, if is_best { BLACK } else { color });
        draw_text(&format!("({},{})", c.x, c.y), 650.0, *y, 14.0, color);
        draw_text(&format_score(c.heuristic), 720.0, *y, 14.0, color);
        draw_text(&format_score(c.score), 800.0, *y, 14.0, color);

        let bar_len = 80.0 * (c.score.abs() as f32 / max_abs as f32);
        draw_rectangle(860.0, *y - 11.0, bar_len, 8.0, color);
        if c.cutoff {
            draw_text("P", 950.0, *y, 14.0, DARKGRAY);
        }
        if c.timed_out {
            draw_text("T", 970.0, *y, 14.0, LIGHTGRAY);
        }
        *y += 18.0;
    }
    *y += 6.0;
    draw_text("P = pruned (alpha-beta)   T = time limit", PANEL_X, *y, 12.0, LIGHTGRAY);
    *y += 16.0;
}

fn format_score(v: i32) -> String {
    if v.abs() >= 1_000_000 {
        format!("{}{:.1}M", if v < 0 { "-" } else { "" }, v.abs() as f64 / 1_000_000.0)
    } else if v.abs() >= 1_000 {
        format!("{}{:.1}K", if v < 0 { "-" } else { "" }, v.abs() as f64 / 1_000.0)
    } else {
        v.to_string()
    }
}

fn format_thousands(n: usize) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out
}
