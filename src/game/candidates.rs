use rustc_hash::FxHashSet;

use crate::game::{game::Game, hex::Hex};

pub struct Candidates {
    set: FxHashSet<Hex>,
    seen: FxHashSet<Hex>, // occupied hexes we've already expanded from
    radius: i32,
}

impl Candidates {
    pub fn new(radius: i32) -> Self {
        Self {
            set: FxHashSet::default(),
            seen: FxHashSet::default(),
            radius,
        }
    }

    pub fn sync(&mut self, game: &Game) {
        for &hex in game.board.cells.keys() {
            if self.seen.insert(hex) {
                self.set.remove(&hex);
                self.expand(game, hex);
            }
        }
    }

    fn expand(&mut self, game: &Game, origin: Hex) {
        let r = self.radius;
        for dq in -r..=r {
            let dr_min = (-r).max(-dq - r);
            let dr_max = r.min(-dq + r);
            for dr in dr_min..=dr_max {
                let candidate = Hex(origin.0 + dq, origin.1 + dr);
                if game.board.is_empty(&candidate)
                    && Hex::origin().distance(candidate) <= game.cfg.size_limit
                {
                    self.set.insert(candidate);
                }
            }
        }
    }

    pub fn as_vec(&self) -> Vec<Hex> {
        self.set.iter().copied().collect()
    }
}
