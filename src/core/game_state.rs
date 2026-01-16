#[derive(Clone)]
pub struct GameState {
    pub board: [[u8; 18]; 18],
    pub captures: [u32; 2],
    pub turn_count: u32,
}

impl GameState {
}
