use crate::game::{hex::Hex, player::Player};
use rustc_hash::FxHasher;
use std::hash::{Hash, Hasher};

pub fn hash(hex: Hex, player: Player) -> u64 {
    let mut hasher = FxHasher::default();
    hex.hash(&mut hasher);
    player.hash(&mut hasher);
    hasher.finish()
}
