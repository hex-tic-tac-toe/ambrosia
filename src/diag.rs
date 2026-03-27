use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Diag {
    GameOver,
    InvalidTurn,
    CellOutOfBounds,
    CellOccupied,
}

impl std::error::Error for Diag {}
impl Display for Diag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Diag::GameOver => write!(f, "Game over"),
            Diag::InvalidTurn => write!(f, "Invalid turn"),
            Diag::CellOutOfBounds => write!(f, "Cell out of bounds"),
            Diag::CellOccupied => write!(f, "Cell occupied"),
        }
    }
}
