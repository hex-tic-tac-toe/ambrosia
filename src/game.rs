use crate::{
    board::{Board, Cell},
    coords::Hex,
    diag::Diag,
};

#[derive(Debug, Clone, Copy)]
pub enum Turn {
    One(Hex),
    Two(Hex, Hex),
}

impl Turn {
    pub fn len(&self) -> usize {
        match self {
            Turn::One(_) => 1,
            Turn::Two(_, _) => 2,
        }
    }

    pub fn iter(&self) -> Box<dyn Iterator<Item = Hex> + '_> {
        match self {
            Turn::One(h) => Box::new(std::iter::once(*h)),
            Turn::Two(a, b) => Box::new(std::iter::once(*a).chain(std::iter::once(*b))),
        }
    }
}

pub enum GameResult {
    Winner(Cell),
    Ongoing,
    Draw,
}

pub struct Game {
    pub board: Board,
    pub to_move: Cell,
    pub turn_number: u32,
    pub result: GameResult,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            to_move: Cell::X,
            turn_number: 0,
            result: GameResult::Ongoing,
        }
    }

    pub fn is_over(&self) -> bool {
        matches!(self.result, GameResult::Winner(_) | GameResult::Draw)
    }

    pub fn placements_this_turn(&self) -> usize {
        if self.turn_number == 0 { 1 } else { 2 }
    }

    pub fn apply(&mut self, turn: Turn) -> Result<(), Diag> {
        if self.is_over() {
            return Err(Diag::GameOver);
        }

        let ex = self.placements_this_turn();
        if turn.len() != ex {
            return Err(Diag::InvalidTurn);
        }

        if let Turn::Two(a, b) = turn
            && a == b
        {
            return Err(Diag::InvalidTurn);
        }

        for hex in turn.iter() {
            // Allow the very first placement at Hex::ZERO even though the
            // board's candidate set is empty. Other placements must be
            // validated via `is_legal_placement`.
            if self.turn_number == 0 {
                if hex != Hex::ZERO {
                    if !self.board.is_empty(hex) {
                        return Err(Diag::CellOccupied);
                    } else {
                        return Err(Diag::CellOutOfBounds);
                    }
                } else {
                    // hex == ZERO is allowed on empty board
                    continue;
                }
            }

            if !self.board.is_legal_placement(hex) {
                if !self.board.is_empty(hex) {
                    return Err(Diag::CellOccupied);
                } else {
                    return Err(Diag::CellOutOfBounds);
                }
            }
        }

        for hex in turn.iter() {
            self.board.place(hex, self.to_move);
            if self.board.is_winning_move(hex, self.to_move) {
                self.result = GameResult::Winner(self.to_move);
                break;
            }
        }

        self.turn_number += 1;
        self.to_move = self.to_move.opponent();

        Ok(())
    }

    pub fn winner(&self) -> Option<Cell> {
        match self.result {
            GameResult::Winner(winner) => Some(winner),
            _ => None,
        }
    }
}
