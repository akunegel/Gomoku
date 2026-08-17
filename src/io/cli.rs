use crate::core::GameState;
use super::interface::{Interface, PlayerAction};
use std::pin::Pin;
use std::future::Future;
pub struct CliInterface;

impl Interface for CliInterface {
    fn render(&mut self, state: &GameState) {
        print!("   ");
        for x in 0..19 {
            print!("{:02} ", x); 
        }
        println!();

        for y in 0..19 {
            print!("{:02} ", y);
            for x in 0..19 {
                if state.hint_move == Some((x, y)) {
                    print!(" H ");
                } else {
                    match state.board[y][x] {
                    0 => print!(" . "),
                    1 => print!(" X "),
                    2 => print!(" O "),
                    _ => print!(" ? "),
                    }
                } 
            }
            println!();
        }
        println!("\n--- Score ---");
        println!("Black (X) Captures: {}", state.captures[0]);
        println!("White (O) Captures: {}", state.captures[1]);
        if state.last_ai_time > 0.0 {
            println!("AI Time: {:.4}s", state.last_ai_time);
        }
        println!("\n--- Moves ---");
        for m in state.move_history() {
            let player = if m.player == 1 { "B" } else { "W" };
            if m.captures > 0 {
                println!("{}. {} ({}, {}) +{}", m.number, player, m.x, m.y, m.captures);
            } else {
                println!("{}. {} ({}, {})", m.number, player, m.x, m.y);
            }
        }
        if let Some(winner) = state.winner {
            println!("\n====================================");
            println!("      GAME OVER! Winner: {}", if winner == 1 { "BLACK (X)" } else { "WHITE (O)" });
            println!("====================================");
        }
        println!("----------------\n");
    }

    fn get_action(&mut self, _state: &GameState) -> Option<PlayerAction> {
        println!("Enter your move (x y), z to undo, s to save, q to quit: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        match input {
            "z" | "undo" => return Some(PlayerAction::Undo),
            "s" | "save" => return Some(PlayerAction::Save),
            "q" | "quit" => return Some(PlayerAction::Quit),
            _ => {}
        }
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 2 {
            return None;
        }
        let x = parts[0].parse::<usize>().ok()?;
        let y = parts[1].parse::<usize>().ok()?;
        Some(PlayerAction::Place((x, y)))
    }

    fn is_key_pressed(&self, _key: char) -> bool {
        false
    }
    fn wait(&mut self) -> Pin<Box<dyn Future<Output = ()> + '_>> {
        Box::pin(async {})
    }

    fn get_save_path(&mut self) -> Pin<Box<dyn Future<Output = Option<String>> + '_>> {
        Box::pin(async {
            println!("Save path: ");
            let mut path = String::new();
            std::io::stdin().read_line(&mut path).expect("Failed to read line");
            let path = path.trim().to_string();
            if path.is_empty() { None } else { Some(path) }
        })
    }

    fn wait_for_continue<'a>(&'a mut self, _state: &'a GameState) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
        Box::pin(async move {
            println!("Press Enter to continue...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
        })
    }
}
