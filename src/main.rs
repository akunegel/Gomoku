extern crate macroquad;
mod core;
mod io;

use std::io as stdio;
use core::GameState;
use io::{Interface, CliInterface, GuiInterface};

use crate::core::zobrist::Zobrist;


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
    let mode = loop {
        println!("Select Mode: (1) Human vs Human [PVP], (2) Human vs AI [PVA]");
        let mut choice = String::new();
        stdio::stdin().read_line(&mut choice).expect("Failed");
        match choice.trim() {
            "1" => break core::game_state::GameMode::PVP,
            "2" => break core::game_state::GameMode::PVA,
            _ => println!("Invalid choice."),
        }
    };
    let mut state = GameState::new(mode);
    let zobrist = Zobrist::new();
    game_loop(&mut state, interface.as_mut(), &zobrist).await;
}

async fn game_loop(state: &mut GameState, interface: &mut dyn Interface, zobrist: &Zobrist) {
    loop {
        interface.render(state);

        if state.winner.is_none() {
            if interface.is_key_pressed('H') {
                println!("Hint requested. AI is thinking...");
                state.hint_move = core::ai::minimax::find_best_move(state, zobrist);
            }
            let current_p = state.current_player();

            let maybe_move = match state.mode {
                core::game_state::GameMode::PVP => {
                    interface.get_move(state)
                }
                core::game_state::GameMode::PVA => {
                    if current_p == 1 {
                        interface.get_move(state)
                    } else {
                        println!("AI is thinking...");
                        let start_time = std::time::Instant::now();
                        let res = core::ai::minimax::find_best_move(state, zobrist);
                        let duration = start_time.elapsed();
                        state.last_ai_time = duration.as_secs_f64();
                        res
                    }
                }
            };

            if let Some((x, y)) = maybe_move {
                match state.can_place_piece(x, y) {
                    Ok(()) => {
                        state.place_piece(x, y, zobrist);
                        state.hint_move = None;

                        if let Some(w) = state.winner {
                            println!("Game Over! Player {} won!", w);
                        }
                    },
                    Err(e) => println!("Invalid move: {}", e),
                }
            }
        }

        interface.wait().await;
    }
}
