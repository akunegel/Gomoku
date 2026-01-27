use crate::core::GameState;
use std::pin::Pin;
use std::future::Future;

pub trait Interface {
    fn render(&mut self, state: &GameState);
    fn get_move(&mut self, state: &GameState) -> Option<(usize, usize)>;
    fn wait(&mut self) -> Pin<Box<dyn Future<Output = ()> + '_>>;
}
