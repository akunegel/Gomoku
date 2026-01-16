mod core;
mod io;

use std::io as stdio;
use core::GameState;
use io::{Interface, CliInterface, GuiInterface};

fn main() {
    println!("Do you want to play CLI gomoku (1) or GUI gomoku (2)?");
    let mut choice = String::new();
    stdio::stdin().read_line(&mut choice).expect("Failed to read line");

    let mut state = GameState::new();

    let interface: Box<dyn Interface> = match choice.trim() {
        "2" => Box::new(GuiInterface),
        _ => Box::new(CliInterface),
    };

    game_loop(&mut state, interface.as_ref());
}

fn game_loop(state: &mut GameState, interface: &dyn Interface) {
    loop {
        interface.render(state);

        if let Some((x, y)) = interface.get_move(state) {
            if x < 19 && y < 19 && state.board[y][x] == 0 {
                state.place_piece(x, y);
            }
        }
    }
}
