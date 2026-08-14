use rand::Rng;

#[derive(Clone)]
pub struct Zobrist {
    pub table: [[u64; 2]; 19 * 19],
    pub turn_hash: [u64; 2],
}

impl Zobrist {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let mut table = [[0u64; 2]; 19 * 19];

        for cell in table.iter_mut() {
            cell[0] = rng.random();
            cell[1] = rng.random();
        }
        let turn_hash = [rng.random(), rng.random()];

        Zobrist { table, turn_hash }
    }

    pub fn get_value(&self, x: usize, y: usize, player: u8) -> u64 {
        if player == 0 {
            return 0;
        }

        let idx = y * 19 + x;
        let p_idx = (player - 1) as usize;
        self.table[idx][p_idx]
    }
    pub fn get_turn_hash(&self, player: u8) -> u64 {
        let idx = (player - 1) as usize;
        self.turn_hash[idx]
    }
}
