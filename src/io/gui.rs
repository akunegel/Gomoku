use crate::core::GameState;
use super::interface::Interface;

pub struct GuiInterface;

impl Interface for GuiInterface {
    fn render(&self, _state: &GameState) {
        // GUI rendering will be implemented here
    }

    fn get_move(&self, _state: &GameState) -> Option<(usize, usize)> {
        // Get move from GUI input
        None
    }
}
