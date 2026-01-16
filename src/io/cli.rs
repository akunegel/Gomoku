use crate::core::game_state::GameState;
use super::interface::Interface;
use std::pin::Pin;
use std::future::Future;
use std::io::{self, Write};

pub struct CliInterface;

impl Interface for CliInterface {
    fn render(&mut self, state: &GameState) {
        println!("\n--- Board ---");
        for y in 0..19 {
            for x in 0..19 {
                let s = match state.board[y][x] {
                    1 => "X",
                    2 => "O",
                    _ => ".",
                };
                print!("{} ", s);
            }
            println!();
        }
    }

    fn get_move(&mut self, _state: &GameState) -> Option<(usize, usize)> {
        print!("Enter x y: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok()?;
        let parts: Vec<usize> = input.split_whitespace()
            .filter_map(|s| s.parse().ok()).collect();
        if parts.len() == 2 { Some((parts[0], parts[1])) } else { None }
    }

    fn wait(&mut self) -> Pin<Box<dyn Future<Output = ()> + '_>> {
        Box::pin(async {}) // CLIは待機不要なので空
    }
}