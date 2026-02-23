#[derive(Clone, Copy, PartialEq, Debug)]

pub enum NodeType {
    Exact,
    LowerBound,
    UpperBound,
}

#[derive(Clone, Copy, Debug)]

pub struct Entry {
    pub hash: u64,
    pub score: i32,
    pub depth: u32,
    pub node_type: NodeType,
    pub best_move: Option<(usize, usize)>,
}

pub struct TranspositionTable {
    table: Vec<Option<Entry>>,
    size: usize,
}

impl TranspositionTable {
    pub fn new(size_mb: usize) -> Self {
        let num_entries = (size_mb * 1024 * 1024) / std::mem::size_of::<Option<Entry>>();
        let table = vec![None; num_entries];
        TranspositionTable {
            table,
            size: num_entries,
        }
    }

    pub fn save(&mut self, hash: u64, depth: u32, score: i32, node_type: NodeType, best_move: Option<(usize, usize)>) {
        let index = (hash as usize) % self.size;
        if let Some(entry) = &self.table[index] {
            if entry.depth > depth {
                return;
            }
        }
        self.table[index] = Some(Entry {
            hash,
            score,
            depth,
            node_type,
            best_move,
        });
    }

    pub fn get(&self, hash: u64) -> Option<Entry> {
        let index = (hash as usize) % self.size;
        if let Some(entry) = &self.table[index] {
            if entry.hash == hash {
                return Some(*entry);
            }
        }
        None
    }
}

