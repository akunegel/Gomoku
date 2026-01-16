mod core;
mod io;

use core::GameState;
use io::{Interface, CliInterface, GuiInterface};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut state = GameState::new();

    let interface: Box<dyn Interface> = if args.len() > 1 && args[1] == "gui" {
        Box::new(GuiInterface)
    } else {
        Box::new(CliInterface)
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
