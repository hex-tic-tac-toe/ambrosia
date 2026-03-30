use std::{f64, sync::Arc};

use crate::{
    ai::{
        bot::Bot,
        features,
        genome::{FeatureSet, Genome},
    },
    game::{candidates::Candidates, game::Game, hex::Hex, mv::Move},
};

/// A heuristic-based bot that evaluates moves using a set of `Feature`s and a `Genome`.
pub struct HeuristicBot {
    features: FeatureSet<Game>,
    genome: Genome,
    candidates: Candidates,
}

impl HeuristicBot {
    pub fn new() -> Self {
        Self {
            features: FeatureSet {
                features: vec![
                    Arc::new(features::LongestRun),
                    Arc::new(features::ThreatScore),
                    Arc::new(features::OpenThreats(2)),
                    Arc::new(features::DoubleThreats),
                    Arc::new(features::GapThreats),
                    Arc::new(features::OpponentThreat),
                    Arc::new(features::LargestCluster),
                    Arc::new(features::IsolatedPieces),
                    Arc::new(features::CentreProximity),
                ],
            },
            genome: Genome {
                weights: vec![3.0, 1.0, 5.0, 12.0, 4.0, -1.8, 1.5, -2.0, 0.8],
            },
            candidates: Candidates::new(8),
        }
    }

    pub fn evaluate_move<'b>(&self, hex: &Hex, game: &'b mut Game) -> f64 {
        if let Ok(t) = game.apply_half_turn(*hex) {
            let mut score = 0.0;
            for (f, w) in self.features.features.iter().zip(&self.genome.weights) {
                score += f.score(game) * w;
            }
            game.undo_half_turn(t);
            return score;
        }

        f64::NEG_INFINITY
    }
}

impl Bot for HeuristicBot {
    fn name(&self) -> &str {
        "heuristic"
    }

    fn choose(&mut self, game: &mut Game) -> Option<Move> {
        self.candidates.sync(game);
        let candidates = self.candidates.as_vec();
        if candidates.is_empty() {
            return None;
        }

        let mut scored_moves: Vec<(Hex, f64)> = candidates
            .into_iter()
            .map(|hex| (hex, self.evaluate_move(&hex, game)))
            .collect();

        scored_moves.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        Some(Move(scored_moves.get(0)?.0, scored_moves.get(1)?.0))
    }
}
