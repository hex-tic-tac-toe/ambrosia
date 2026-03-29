use rustc_hash::FxHashMap;

use crate::game::{hex::Hex, player::Player};

pub struct Board {
    pub cells: FxHashMap<Hex, Player>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: FxHashMap::default(),
        }
    }

    pub fn is_empty(&self, hex: &Hex) -> bool {
        !self.cells.contains_key(hex)
    }

    pub fn get(&self, hex: &Hex) -> Option<&Player> {
        self.cells.get(hex)
    }

    pub fn place(&mut self, hex: Hex, player: Player) {
        self.cells.insert(hex, player);
    }
}
