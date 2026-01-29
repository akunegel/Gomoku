use crate::core::GameState;
use super::interface::Interface;
use std::pin::Pin;
use std::future::Future;
pub struct CliInterface;

impl Interface for CliInterface {
    fn render(&mut self, state: &GameState) {
        print!("");
        for x in 0..19 {
            print!("{:02} ", x); 
        }
        println!();

        for x in 0..19 {
            for y in 0..19 {
                match state.board[y][x] {
                    0 => print!(" . "),
                    1 => print!(" X "),
                    2 => print!(" O "),
                    _ => print!(" ? "),
                }
            }
            println!();
        }
        println!("\n--- Score ---");
        println!("Black (X) Captures: {}", state.captures[0]);
        println!("White (O) Captures: {}", state.captures[1]);
        println!("----------------\n");
    }

    fn get_move(&mut self, _state: &GameState) -> Option<(usize, usize)> {
        println!("Enter your move (x y): ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() != 2 {
            return None;
        }
        let x = parts[0].parse::<usize>().ok()?;
        let y = parts[1].parse::<usize>().ok()?;
        Some((x, y))
    }
    fn wait(&mut self) -> Pin<Box<dyn Future<Output = ()> + '_>> {
        Box::pin(async {})
    }
}
