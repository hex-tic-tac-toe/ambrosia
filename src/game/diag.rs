#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Diag {
    CellOccupied,
    CannotPlaceTwice,
    TooFarAway,
}
