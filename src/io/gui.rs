use super::interface::Interface;
use crate::core::GameState;

pub struct GuiInterface;

impl Interface for GuiInterface {
    fn render(&self, _state: &GameState) {
        // GUI rendering will be implemented here
    }

    fn get_move(&self, _state: &mut GameState) -> Option<(usize, usize)> {
        // Get move from GUI input
        None
    }
}
