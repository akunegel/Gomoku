extern crate macroquad;
mod core;
mod io;

use std::io as stdio;
use core::GameState;
use io::{Interface, CliInterface, GuiInterface};


#[macroquad::main("Gomoku")]
async fn main() {
    let mut interface: Box<dyn Interface> = loop {
        println!("Do you want to play CLI gomoku (1) or GUI gomoku (2)?");
        let mut choice = String::new();
        stdio::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => break Box::new(CliInterface),
            "2" => break Box::new(GuiInterface),
            _ => println!("Invalid choice. Please enter 1 or 2.\n"),
        }
    };

    let mut state = GameState::new();
    game_loop(&mut state, interface.as_mut()).await;
}

async fn game_loop(state: &mut GameState, interface: &mut dyn Interface) {
    loop {
        interface.render(state);

        if let Some(winner) = state.check_win() {
            let player_name = if winner == 1 { "BLACK" } else { "WHITE" };
            println!("Game Over! Winner is: {}!", player_name);
            interface.render(state);
            break;
        }

        if let Some((x, y)) = interface.get_move(state) {
            if x < 19 && y < 19 && state.board[y][x] == 0 {
                state.place_piece(x, y);
            }
        }
        interface.wait().await;
    }
}