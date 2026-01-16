use crate::core::GameState;

pub trait Interface {
    fn render(&self, state: &GameState);
    fn get_move(&self, state: &GameState) -> Option<(usize, usize)>;
}
