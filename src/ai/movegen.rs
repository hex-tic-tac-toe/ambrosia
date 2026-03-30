use rustc_hash::FxHashSet;

use crate::game::{game::Game, hex::Hex};

pub fn generate_candidates(game: &Game, radius: i32) -> Vec<Hex> {
    let mut set = FxHashSet::default();

    for &hex in game.board.cells.keys() {
        let (hq, hr) = (hex.0, hex.1);
        for dq in -radius..=radius {
            let dr_min = (-radius).max(-dq - radius);
            let dr_max = radius.min(-dq + radius);
            for dr in dr_min..=dr_max {
                let (cq, cr) = (hq + dq, hr + dr);
                let dist_from_origin = (cq.abs() + cr.abs() + (cq + cr).abs()) / 2;
                if dist_from_origin <= game.cfg.size_limit
                    && game.board.cells.get(&Hex(cq, cr)).is_none()
                {
                    set.insert(Hex(cq, cr));
                }
            }
        }
    }

    set.into_iter().collect()
}
