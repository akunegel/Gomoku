use crate::core::GameState;
use std::pin::Pin;
use std::future::Future;

pub enum PlayerAction {
    Place((usize, usize)),
    Undo,
    Save,
    Quit,
}

pub trait Interface {
    fn render(&mut self, state: &GameState);
    fn get_action(&mut self, state: &GameState) -> Option<PlayerAction>;
    fn wait(&mut self) -> Pin<Box<dyn Future<Output = ()> + '_>>;
    fn is_key_pressed(&self, key: char) -> bool;
    fn get_save_path(&mut self) -> Pin<Box<dyn Future<Output = Option<String>> + '_>>;
    fn visualizer_enabled(&self) -> bool {
        false
    }
}
