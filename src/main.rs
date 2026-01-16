extern crate macroquad;
mod core;
mod io;

use core::GameState;
use io::{Interface, CliInterface, GuiInterface};

#[macroquad::main("Gomoku")]
async fn main() {
    macroquad::window::next_frame().await;
    let args: Vec<String> = std::env::args().collect();

    let mut state = GameState::new();

    let mut interface: Box<dyn Interface> = if args.len() > 1 && args[1] == "gui" {
        Box::new(GuiInterface)
    } else {
        Box::new(CliInterface)
    };

    game_loop(&mut state, interface.as_mut()).await;
}

async fn game_loop(state: &mut GameState, interface: &mut dyn Interface) {
    loop {
        interface.render(state);
        interface.wait().await;

        if let Some((x, y)) = interface.get_move(state) {
            if x < 19 && y < 19 && state.board[y][x] == 0 {
                state.place_piece(x, y);
            }
        }
    }
}
