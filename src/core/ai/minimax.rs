use crate::core::GameState;
use crate::core::zobrist::Zobrist;
use crate::core::tt::{TranspositionTable, NodeType};
use std::time::{Instant, Duration};
use std::sync::{Arc, Mutex};

use super::search::{get_candidates, move_heuristic};
use super::evaluation::evaluate_board;
use super::report::{RootCandidate, SearchProgress, SearchReport, SearchStats, SharedProgress};

const TIME_LIMIT_MS: u64 = 900;
const SEARCH_DEPTH: u32 = 10;
const MIN_SCORE: i32 = -200_000_000;
const MAX_SCORE: i32 = 200_000_000;
const FLUSH_EVERY: u64 = 512;

pub fn find_best_move(state: &GameState, zobrist: &Zobrist) -> Option<(usize, usize)> {
    let progress = Arc::new(Mutex::new(SearchProgress::new(SEARCH_DEPTH)));
    search_with_progress(state, zobrist, &progress).0
}

pub fn search_with_progress(
    state: &GameState,
    zobrist: &Zobrist,
    progress: &SharedProgress,
) -> (Option<(usize, usize)>, SearchReport) {
    let start_time = Instant::now();
    let time_limit = Duration::from_millis(TIME_LIMIT_MS);
    let player = state.current_player();
    let mut stats = SearchStats::new();
    let mut tt = TranspositionTable::new(256);

    {
        let mut p = progress.lock().unwrap();
        p.depth = SEARCH_DEPTH;
        p.started = true;
    }

    let best = if state.board.iter().flatten().all(|&cell| cell == 0) {
        Some((9, 9))
    } else {
        search_at_depth(
            state,
            SEARCH_DEPTH,
            MIN_SCORE,
            MAX_SCORE,
            zobrist,
            &mut tt,
            &start_time,
            time_limit,
            &mut stats,
            progress,
        )
        .or_else(|| {
            for y in 0..19 {
                for x in 0..19 {
                    if state.board[y][x] == 0 && state.can_place_piece(x, y).is_ok() {
                        return Some((x, y));
                    }
                }
            }
            None
        })
    };

    let elapsed_ms = start_time.elapsed().as_millis();
    let timed_out;
    let candidates;
    {
        let mut p = progress.lock().unwrap();
        timed_out = p.timed_out;
        for rc in p.root_results.iter_mut() {
            rc.is_best = best.map(|m| m == (rc.x, rc.y)).unwrap_or(false);
        }
        candidates = p.root_results.clone();
    }

    let best_score = candidates
        .iter()
        .find(|c| c.is_best)
        .map(|c| c.score)
        .unwrap_or(0);

    let report = SearchReport {
        player,
        best_move: best,
        best_score,
        elapsed_ms,
        nodes: stats.nodes,
        tt_hits: stats.tt_hits,
        depth: SEARCH_DEPTH,
        timed_out,
        candidates,
    };
    (best, report)
}

fn search_at_depth(
    state: &GameState,
    depth: u32,
    mut alpha: i32,
    mut beta: i32,
    zobrist: &Zobrist,
    tt: &mut TranspositionTable,
    start_time: &Instant,
    time_limit: Duration,
    stats: &mut SearchStats,
    progress: &SharedProgress,
) -> Option<(usize, usize)> {
    let is_maximizing = state.current_player() == 1;
    let mut candidates: Vec<(i32, usize, usize)> = get_candidates(state)
        .into_iter()
        .map(|(x, y)| (move_heuristic(state, x, y), x, y))
        .collect();
    candidates.sort_by_key(|&(h, _, _)| -h);

    {
        let mut p = progress.lock().unwrap();
        p.total_candidates = candidates.len();
        p.best_move_so_far = None;
        p.best_score_so_far = if is_maximizing { MIN_SCORE } else { MAX_SCORE };
    }

    let mut best_move = candidates.first().map(|&(_, x, y)| (x, y));
    let mut best_score = if is_maximizing { MIN_SCORE } else { MAX_SCORE };

    let max_branches = match depth {
        1..=3 => 12,
        4..=6 => 8,
        _ => 5,
    };

    for (i, (heuristic, x, y)) in candidates.into_iter().take(max_branches).enumerate() {
        if start_time.elapsed() >= time_limit {
            progress.lock().unwrap().timed_out = true;
            break;
        }

        {
            let mut p = progress.lock().unwrap();
            p.current_candidate = Some((x, y));
        }

        let mut next_state = state.clone();
        next_state.place_piece(x, y, zobrist);

        let score = alpha_beta(
            &next_state,
            depth - 1,
            alpha,
            beta,
            !is_maximizing,
            zobrist,
            tt,
            start_time,
            time_limit,
            stats,
            progress,
        );
        let timed_out = start_time.elapsed() >= time_limit;

        if timed_out {
            progress.lock().unwrap().timed_out = true;
            break;
        }

        let mut cutoff = false;
        if is_maximizing {
            if score > best_score {
                best_score = score;
                best_move = Some((x, y));
            }
            alpha = alpha.max(score);
        } else {
            if score < best_score {
                best_score = score;
                best_move = Some((x, y));
            }
            beta = beta.min(score);
        }
        if alpha >= beta {
            cutoff = true;
        }

        {
            let mut p = progress.lock().unwrap();
            p.candidates_done = i + 1;
            p.current_candidate = None;
            p.best_move_so_far = best_move;
            p.best_score_so_far = best_score;
            p.root_results.push(RootCandidate {
                x,
                y,
                heuristic,
                score,
                is_best: false,
                timed_out,
                cutoff,
            });
        }

        if cutoff {
            break;
        }
    }

    best_move
}

fn alpha_beta(
    state: &GameState,
    depth: u32,
    mut alpha: i32,
    mut beta: i32,
    is_maximizing: bool,
    zobrist: &Zobrist,
    tt: &mut TranspositionTable,
    start_time: &Instant,
    time_limit: Duration,
    stats: &mut SearchStats,
    progress: &SharedProgress,
) -> i32 {
    if start_time.elapsed() >= time_limit {
        progress.lock().unwrap().timed_out = true;
        return -999_999_999;
    }

    stats.nodes += 1;
    if stats.nodes % FLUSH_EVERY == 0 {
        flush_stats(progress, stats);
    }

    if let Some(winner) = state.winner {
        return if winner == 1 { 100_000_000 + depth as i32 } else { -100_000_000 - depth as i32 };
    }
    if depth == 0 {
        return evaluate_board(state);
    }

    let mut tt_best_move = None;
    if let Some(entry) = tt.get(state.hash) {
        stats.tt_hits += 1;
        tt_best_move = entry.best_move;
        if entry.depth >= depth {
            match entry.node_type {
                NodeType::Exact => return entry.score,
                NodeType::LowerBound => alpha = alpha.max(entry.score),
                NodeType::UpperBound => beta = beta.min(entry.score),
            }
            if alpha >= beta { return entry.score; }
        }
    }

    let alpha_orig = alpha;
    let mut best_val = if is_maximizing { MIN_SCORE } else { MAX_SCORE };
    let mut current_best_move = None;

    let mut candidates: Vec<(i32, usize, usize)> = get_candidates(state)
        .into_iter()
        .map(|(x, y)| (move_heuristic(state, x, y), x, y))
        .collect();
    candidates.sort_by_key(|&(h, _, _)| -h);

    if let Some(m) = tt_best_move {
        if let Some(pos) = candidates.iter().position(|&(_, x, y)| (x, y) == m) {
            let cand = candidates.remove(pos);
            candidates.insert(0, cand);
        }
    }

    let max_branches = if depth >= 6 { 4 } else if depth >= 3 { 7 } else { 5 };

    for (_, x, y) in candidates.into_iter().take(max_branches) {
        if state.can_place_piece(x, y).is_ok() {
            let mut next_state = state.clone();
            next_state.place_piece(x, y, zobrist);
            let eval = alpha_beta(
                &next_state,
                depth - 1,
                alpha,
                beta,
                !is_maximizing,
                zobrist,
                tt,
                start_time,
                time_limit,
                stats,
                progress,
            );

            if is_maximizing {
                if eval > best_val {
                    best_val = eval;
                    current_best_move = Some((x, y));
                }
                alpha = alpha.max(eval);
            } else {
                if eval < best_val {
                    best_val = eval;
                    current_best_move = Some((x, y));
                }
                beta = beta.min(eval);
            }
            if beta <= alpha { break; }
        }
    }

    let node_type = if best_val <= alpha_orig { NodeType::UpperBound }
                    else if best_val >= beta { NodeType::LowerBound }
                    else { NodeType::Exact };
    tt.save(state.hash, depth, best_val, node_type, current_best_move);
    best_val
}

fn flush_stats(progress: &SharedProgress, stats: &SearchStats) {
    let mut p = progress.lock().unwrap();
    p.nodes = stats.nodes;
    p.tt_hits = stats.tt_hits;
}
