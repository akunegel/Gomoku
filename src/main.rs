mod core;
mod io;

use core::GameState;
use io::{CliInterface, GuiInterface, Interface};
use std::io as stdio;

fn main() {
    let interface: Box<dyn Interface> = loop {
        println!("Do you want to play CLI gomoku (1) or GUI gomoku (2)?");
        let mut choice = String::new();
        stdio::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => break Box::new(CliInterface),
            "2" => break Box::new(GuiInterface),
            _ => println!("Invalid choice. Please enter 1 or 2.\n"),
        }
    };

    let mut state = GameState::new();
    game_loop(&mut state, interface.as_ref());
}

fn game_loop(state: &mut GameState, interface: &dyn Interface) {
    loop {
        interface.render(state);

        if let Some((x, y)) = interface.get_move(state) {
            if x < 19 && y < 19 && state.board[y][x] == 0 {
                state.place_piece(x, y);
            }

            if let Some(winner) = state.check_win() {
                interface.render(state);
                println!("\nPlayer {} wins!", winner);
                print_move_history(state);
                break;
            }
        }
    }
}

fn print_move_history(state: &GameState) {
    println!("\nFull move history:");
    for (i, &(x, y, player)) in state.moves.iter().enumerate() {
        println!("  {}. Player {} → ({}, {})", i + 1, player, x, y);
    }
}
