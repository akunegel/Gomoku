use std::sync::Mutex;

#[derive(Clone)]
pub struct RootCandidate {
    pub x: usize,
    pub y: usize,
    pub heuristic: i32,
    pub score: i32,
    pub is_best: bool,
    pub timed_out: bool,
    pub cutoff: bool,
}

pub struct SearchStats {
    pub nodes: u64,
    pub tt_hits: u64,
}

impl SearchStats {
    pub fn new() -> Self {
        SearchStats { nodes: 0, tt_hits: 0 }
    }
}

#[derive(Clone)]
pub struct SearchProgress {
    pub started: bool,
    pub current_candidate: Option<(usize, usize)>,
    pub candidates_done: usize,
    pub total_candidates: usize,
    pub nodes: u64,
    pub tt_hits: u64,
    pub depth: u32,
    pub root_results: Vec<RootCandidate>,
    pub best_move_so_far: Option<(usize, usize)>,
    pub best_score_so_far: i32,
    pub timed_out: bool,
}

impl SearchProgress {
    pub fn new(depth: u32) -> Self {
        SearchProgress {
            started: false,
            current_candidate: None,
            candidates_done: 0,
            total_candidates: 0,
            nodes: 0,
            tt_hits: 0,
            depth,
            root_results: Vec::new(),
            best_move_so_far: None,
            best_score_so_far: 0,
            timed_out: false,
        }
    }
}

pub type SharedProgress = std::sync::Arc<Mutex<SearchProgress>>;

#[derive(Clone)]
pub struct SearchReport {
    pub player: u8,
    pub best_move: Option<(usize, usize)>,
    pub best_score: i32,
    pub elapsed_ms: u128,
    pub nodes: u64,
    pub tt_hits: u64,
    pub depth: u32,
    pub timed_out: bool,
    pub candidates: Vec<RootCandidate>,
}
