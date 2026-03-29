use crate::game::{hex::Hex, phase::Phase, player::Player};

pub struct Undo {
    pub hex: Hex,
    pub prev_pending: Option<Hex>,
    pub prev_phase: Phase,
    pub prev_turn: Player,
    pub prev_zhash: u64,
}
