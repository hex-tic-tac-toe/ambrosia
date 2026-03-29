use crate::{
    ai::{bot::Bot, movegen},
    game::{config::Config, game::Game, hex::Hex, mv::Move, player::Player},
};
use rand::seq::SliceRandom;
use rand::{Rng, rng};
use rand::{RngExt, rngs::ThreadRng};

/// A heuristic-based bot that evaluates moves using strategic principles
pub struct HeuristicBot {
    center_weight: f64,
    progress_weight: f64,
    block_weight: f64,
    mobility_weight: f64,
}

impl Default for HeuristicBot {
    fn default() -> Self {
        // These weights were chosen empirically - they favor progress and blocking
        Self {
            center_weight: 0.2,
            progress_weight: 0.4,
            block_weight: 0.3,
            mobility_weight: 0.1,
        }
    }
}

impl HeuristicBot {
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate how close a hex is to the center of the board
    fn center_proximity(&self, hex: &Hex, config: &Config) -> f64 {
        // Normalize distance from center (0 at center, increases outward)
        let max_dist = config.size_limit as f64;
        let dist = hex.distance(Hex::origin()) as f64;
        1.0 - (dist / max_dist)
    }

    /// Calculate progress toward the player's goal
    /// For X: progress is negative r coordinate (top to bottom)
    /// For O: progress is positive q coordinate (left to right)
    fn progress_toward_goal(&self, hex: &Hex, player: Player, config: &Config) -> f64 {
        match player {
            Player::X => {
                // X tries to connect top (negative r) to bottom (positive r)
                // Reward negative r values (moving toward bottom)
                let max_r = config.size_limit as f64;
                let normalized_r = (hex.1 + max_r as i32) as f64 / (2.0 * max_r);
                normalized_r
            }
            Player::O => {
                // O tries to connect left (negative q) to right (positive q)
                // Reward positive q values (moving toward right)
                let max_q = config.size_limit as f64;
                let normalized_q = (hex.0 + max_q as i32) as f64 / (2.0 * max_q);
                normalized_q
            }
        }
    }

    /// Calculate how much a move blocks the opponent's progress
    fn block_opponent(&self, hex: &Hex, game: &Game) -> f64 {
        // Simple blocking heuristic: count opponent pieces nearby
        let opponent = game.turn.opponent();
        let mut block_score = 0.0;

        // Check adjacent hexes for opponent pieces
        for dir in Hex::directions().iter() {
            let adjacent = *hex + *dir;
            if let Some(&p) = game.board.get(&adjacent) {
                if p == opponent {
                    block_score += 1.0;
                }
            }
        }

        // Normalize by max possible (6 adjacent hexes)
        block_score / 6.0
    }

    /// Calculate mobility - number of available moves after this move
    fn mobility(&self, hex: &Hex, game: &mut Game) -> f64 {
        let undo = game.apply_half_turn(*hex);
        if undo.is_err() {
            return 0.0; // Invalid move
        }

        let candidates = movegen::generate_candidates(game, game.cfg.view_distance);
        game.undo_half_turn(undo.unwrap());
        candidates.len() as f64
    }

    /// Evaluate a move using our heuristic function
    fn evaluate_move(&self, hex: &Hex, game: &mut Game) -> f64 {
        let player = game.turn;
        let config = game.cfg;

        let center_score = self.center_proximity(hex, config);
        let progress_score = self.progress_toward_goal(hex, player, config);
        let block_score = self.block_opponent(hex, game);
        let mobility_score = self.mobility(hex, game);

        // Normalize mobility score (rough estimate)
        let normalized_mobility = (mobility_score / 20.0).min(1.0).max(0.0);

        self.center_weight * center_score
            + self.progress_weight * progress_score
            + self.block_weight * block_score
            + self.mobility_weight * normalized_mobility
    }
}

impl Bot for HeuristicBot {
    fn name(&self) -> &str {
        "heuristic"
    }

    fn choose(&self, game: &mut Game) -> Option<Move> {
        let mut candidates = movegen::generate_candidates(game, game.cfg.view_distance);

        if candidates.is_empty() {
            return None;
        }

        // Evaluate all candidate moves
        let mut scored_moves: Vec<(Hex, f64)> = candidates
            .into_iter()
            .map(|hex| (hex, self.evaluate_move(&hex, game)))
            .collect();

        // Sort by score (descending)
        scored_moves.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Add some randomness to avoid always picking the same move
        // Take top 3 and choose randomly from them
        let top_count = (scored_moves.len().min(3)).max(1);
        let top_moves = &scored_moves[..top_count];

        let mut rng = rng();
        let chosen_index = rng.random_range(0..top_moves.len());

        let (first_hex, _) = top_moves[chosen_index];

        // For Hex, we need to return a Move (pair of hexes)
        // Second hex can be another good move or just a placeholder
        let second_candidates: Vec<Hex> = scored_moves
            .iter()
            .filter(|(h, _)| *h != first_hex)
            .map(|(h, _)| *h)
            .collect();

        let second_hex = if !second_candidates.is_empty() {
            *second_candidates.first().unwrap()
        } else {
            first_hex // Fallback - though this would be invalid in practice
        };

        Some(Move(first_hex, second_hex))
    }
}
