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

        if state.winner.is_none() {
            if let Some(winner) = state.check_win() {
                state.winner = Some(winner);
            }
            if let Some((x, y)) = interface.get_move(state) {
                match state.can_place_piece(x, y) {
                    Ok(()) => {
                        state.place_piece(x, y);
                    }
                    Err(e) => {
                        println!("Invalid move: {}", e); 
                    }
                }
            }
        }
        interface.wait().await;
    }
}