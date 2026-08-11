pub mod interface;
pub mod cli;
pub mod gui;

pub use interface::{Interface, PlayerAction};
pub use cli::CliInterface;
pub use gui::GuiInterface;
