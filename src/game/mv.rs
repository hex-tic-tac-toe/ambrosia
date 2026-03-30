use crate::game::hex::Hex;

#[derive(Debug, Clone, Copy)]
pub struct Move(pub Hex, pub Hex);
