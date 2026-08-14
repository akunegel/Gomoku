extern crate macroquad;
mod core;
mod io;

use std::io as stdio;
use core::GameState;
use io::{Interface, CliInterface, GuiInterface, PlayerAction};

use crate::core::zobrist::Zobrist;
use crate::core::ai::{SearchProgress, SharedProgress};
use std::sync::{Arc, Mutex};

fn window_conf() -> macroquad::prelude::Conf {
    macroquad::prelude::Conf {
        window_title: "Gomoku".to_owned(),
        window_width: 1280,
        window_height: 700,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut interface: Box<dyn Interface> = loop {
        println!("Do you want to play CLI gomoku (1) or GUI gomoku (2)?");
        let mut choice = String::new();
        stdio::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => break Box::new(CliInterface),
            "2" => break Box::new(GuiInterface::new(ask_visualizer())),
            _ => println!("Invalid choice. Please enter 1 or 2.\n"),
        }
    };
    let mut state = loop {
        println!("Enter a save file path to load, or press Enter to start a new game:");
        let mut choice = String::new();
        stdio::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim();
        if choice.is_empty() {
            break GameState::new(choose_mode());
        }
        match GameState::load_from_file(choice) {
            Ok(loaded) => break loaded,
            Err(e) => println!("Could not load {}: {}", choice, e),
        }
    };
    let zobrist = Zobrist::new();
    state.recompute_hash(&zobrist);
    game_loop(&mut state, interface.as_mut(), &zobrist).await;
}

fn ask_visualizer() -> bool {
    loop {
        println!("Enable the solver visualizer? (y/n)");
        let mut choice = String::new();
        stdio::stdin().read_line(&mut choice).expect("Failed to read line");
        match choice.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Invalid choice. Please enter y or n.\n"),
        }
    }
}

fn choose_mode() -> core::game_state::GameMode {
    loop {
        println!("Select Mode: (1) Human vs Human [PVP], (2) Human vs AI [PVA], (3) AI vs AI [AVA]");
        let mut choice = String::new();
        stdio::stdin().read_line(&mut choice).expect("Failed");
        match choice.trim() {
            "1" => break core::game_state::GameMode::PVP,
            "2" => break core::game_state::GameMode::PVA,
            "3" => break core::game_state::GameMode::AVA,
            _ => println!("Invalid choice."),
        }
    }
}

async fn game_loop(state: &mut GameState, interface: &mut dyn Interface, zobrist: &Zobrist) {
    loop {
        interface.render(state);

        if interface.is_key_pressed('Z') {
            state.undo_last_move();
        }
        if interface.is_key_pressed('S') {
            save_and_exit(state, interface).await;
        }

        if state.winner.is_none() {
            if interface.is_key_pressed('H') {
                state.hint_move = run_search_live(state, interface, zobrist).await;
            }
        }

        let maybe_action = if state.winner.is_none() {
            match state.mode {
                core::game_state::GameMode::PVP => {
                    interface.get_action(state)
                }
                core::game_state::GameMode::PVA => {
                    if state.current_player() == 1 {
                        interface.get_action(state)
                    } else {
                        run_ai_action(state, interface, zobrist).await
                    }
                }
                core::game_state::GameMode::AVA => {
                    run_ai_action(state, interface, zobrist).await
                }
            }
        } else {
            interface.get_action(state)
        };

        match maybe_action {
            Some(PlayerAction::Place((x, y))) if state.winner.is_none() => {
                match state.can_place_piece(x, y) {
                    Ok(()) => {
                        state.push_history();
                        state.place_piece(x, y, zobrist);
                        state.hint_move = None;

                        if let Some(w) = state.winner {
                            interface.render(state);
                            println!("Game Over! Player {} won!", w);
                        }
                    },
                    Err(e) => {
                        println!("AI attempted an invalid move: {}", e);
            
                        if state.mode == core::game_state::GameMode::AVA {
                            let winner = if state.current_player() == 1 { 2 } else { 1 };
                            state.winner = Some(winner);
                            println!("Game Over! Player {} won by default (Opponent played forbidden move)", winner);
                        }
                    },
                }
            }
            Some(PlayerAction::Undo) => {
                if !state.undo_last_move() {
                    println!("Nothing to undo.");
                }
            }
            Some(PlayerAction::Save) => save_and_exit(state, interface).await,
            Some(PlayerAction::Quit) => return,
            _ => {}
        }
        interface.wait().await;
    }
}

async fn save_and_exit(state: &GameState, interface: &mut dyn Interface) {
    if let Some(path) = interface.get_save_path().await {
        match state.save_to_file(&path) {
            Ok(()) => {
                println!("Game saved to {}", path);
                std::process::exit(0);
            }
            Err(e) => println!("Failed to save {}: {}", path, e),
        }
    }
}

async fn run_ai_action(state: &mut GameState, interface: &mut dyn Interface, zobrist: &Zobrist) -> Option<PlayerAction> {
    let start_time = std::time::Instant::now();
    let res = run_search_live(state, interface, zobrist).await;
    state.last_ai_time = start_time.elapsed().as_secs_f64();
    res.map(PlayerAction::Place)
}

async fn run_search_live(state: &mut GameState, interface: &mut dyn Interface, zobrist: &Zobrist) -> Option<(usize, usize)> {
    if !interface.visualizer_enabled() {
        return core::ai::minimax::find_best_move(state, zobrist);
    }

    let search_state = state.clone();
    let search_zobrist = (*zobrist).clone();
    let progress: SharedProgress = Arc::new(Mutex::new(SearchProgress::new(10)));
    state.search_progress = Some(progress.clone());

    let handle = std::thread::spawn(move || {
        core::ai::minimax::search_with_progress(&search_state, &search_zobrist, &progress)
    });

    loop {
        interface.render(state);
        if handle.is_finished() {
            let (mv, report) = handle.join().unwrap();
            state.last_search = Some(report);
            state.search_progress = None;
            interface.render(state);
            return mv;
        }
        interface.wait().await;
    }
}
