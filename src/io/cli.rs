use crate::core::GameState;
use super::interface::Interface;

pub struct CliInterface;

impl Interface for CliInterface {
    fn render(&self, state: &GameState) {
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
    }

    fn get_move(&self, state: &GameState) -> Option<(usize, usize)> {
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
}
