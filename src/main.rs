extern crate macroquad;
mod core;
mod io;

use std::io as stdio;
use core::GameState;
// use std::time::Instant;
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
            // let maybe_move = interface.get_move(state);
            let current_p = state.current_player();

            let maybe_move = if current_p == 1 {
                interface.get_move(state)
            } else {
                println!("AI is thinking...");
                let start_time = std::time::Instant::now();
                let res = core::ai::minimax::find_best_move(state);
                let duration = start_time.elapsed();
                state.last_ai_time = duration.as_secs_f64();
                res
            };

            if let Some((x, y)) = maybe_move {
                match state.can_place_piece(x, y) {
                    Ok(()) => {
                        state.place_piece(x, y);

                        if let Some(pending_winner) = state.five_aligned_winner {
                            if state.is_five_broken(pending_winner) {
                                state.five_aligned_winner = None;
                                println!("The five-in-a-row was broken! Game continues.");
                            } else {
                                state.winner = Some(pending_winner);
                                println!("Game Over! {} won (Five sustained)!", if pending_winner == 1 { "BLACK" } else { "WHITE" });
                            }
                        }

                        if state.captures[0] >= 10 { state.winner = Some(1); }
                        else if state.captures[1] >= 10 { state.winner = Some(2); }

                        if state.winner.is_none() && state.five_aligned_winner.is_none() {
                            if let Some(w) = state.check_win_by_alignment() {
                                state.five_aligned_winner = Some(w);
                                println!("Five in a row! Next player, try to break it!");
                            }
                        }
                    },
                    Err(e) => println!("Invalid move: {}", e),
                }
            }
        }

        interface.wait().await;
    }
}

// async fn game_loop(state: &mut GameState, interface: &mut dyn Interface) {
//     loop {
//         interface.render(state);

//         if state.winner.is_none() {
//             if let Some(pending_winner) = state.five_aligned_winner {
//                 if state.check_win_by_alignment() == Some(pending_winner) {
//                     state.winner = Some(pending_winner);
//                     println!("Game Over! {} won (Five in a row sustained)!", if pending_winner == 1 { "BLACK" } else { "WHITE" });
//                 } else {
//                     state.five_aligned_winner = None;
//                     println!("The five-in-a-row was broken! Game continues.");
//                 }
//             }
//             if state.captures[0] >= 10 { state.winner = Some(1); }
//             if state.captures[1] >= 10 { state.winner = Some(2); }

//             if state.winner.is_none() {
//                 let current_p = state.current_player();
//                 if current_p == 1 {
//                     if let Some((x, y)) = interface.get_move(state) {
//                         match state.can_place_piece(x, y) {
//                             Ok(()) => {
//                                 state.place_piece(x, y);
//                                 if state.five_aligned_winner.is_none() {
//                                 state.five_aligned_winner = state.check_win_by_alignment();
//                                 }
//                             },
//                             Err(e) => println!("Invalid move: {}", e),
//                         }
//                     }
//                 } else {
//                     let start_time = Instant::now();
//                     if let Some((x, y)) = core::ai::minimax::find_best_move(state) {
//                         let duration = start_time.elapsed();
//                         state.last_ai_time = duration.as_secs_f64();
//                         state.place_piece(x, y);
//                         if state.five_aligned_winner.is_none() {
//                             state.five_aligned_winner = state.check_win_by_alignment();
//                         }
//                     }
//                 }
//             }
//         }
//         interface.wait().await;
//     }
// }