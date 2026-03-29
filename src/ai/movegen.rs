use rustc_hash::FxHashSet;

use crate::game::{game::Game, hex::Hex};

pub fn generate_candidates(game: &Game, radius: i32) -> Vec<Hex> {
    let mut set = FxHashSet::default();

    for &hex in game.board.cells.keys() {
        for dq in -radius..=radius {
            for dr in -radius..=radius {
                let candidate = Hex(hex.0 + dq, hex.1 + dr);

                // skip occupied
                if game.board.is_empty(&candidate)
                    && hex.distance(candidate) <= radius
                    && Hex::origin().distance(candidate) <= game.cfg.size_limit
                {
                    set.insert(candidate);
                }
            }
        }
    }

    set.into_iter().collect()
}
