use crate::{
    game::{
        board::Board, config::Config, diag::Diag, hex::Hex, phase::Phase, player::Player,
        undo::Undo,
    },
    zobrist,
};

pub struct Game<'a> {
    pub cfg: &'a Config,

    pub board: Board,
    pub turn: Player,

    pub phase: Phase,
    pub pending: Option<Hex>,

    pub zhash: u64,
}

impl<'a> Game<'a> {
    pub fn new(config: &'a Config) -> Self {
        let mut board = Board::new();
        let mut zhash = 0;

        board.place(Hex::origin(), Player::X);
        zhash ^= zobrist::hash(Hex::origin(), Player::X);

        Self {
            cfg: config,

            board,
            turn: Player::O,
            phase: Phase::First,
            pending: None,

            zhash,
        }
    }

    pub fn play_half_turn(&mut self, hex: Hex) -> Result<Option<Player>, Diag> {
        if !self.board.is_empty(&hex) {
            return Err(Diag::CellOccupied);
        }

        if let Some(lhs) = self.pending
            && lhs == hex
        {
            return Err(Diag::CannotPlaceTwice);
        }

        if !self.is_within_range(hex, self.cfg.view_distance) {
            return Err(Diag::TooFarAway);
        }

        match self.phase {
            Phase::First => {
                self.board.place(hex, self.turn);
                self.zhash ^= zobrist::hash(hex, self.turn);
                self.pending = Some(hex);
                self.phase = Phase::Second;

                Ok(None)
            }

            Phase::Second => {
                self.board.place(hex, self.turn);
                self.zhash ^= zobrist::hash(hex, self.turn);

                let first = self.pending.unwrap();
                if self.is_winning_move(first, self.turn) || self.is_winning_move(hex, self.turn) {
                    return Ok(Some(self.turn));
                }

                self.pending = None;
                self.phase = Phase::First;
                self.turn = self.turn.opponent();
                Ok(None)
            }
        }
    }

    fn count_dir(&self, start: Hex, dir: Hex, player: Player) -> i32 {
        let mut count = 0;
        let mut cur = start;

        loop {
            cur = cur + dir;

            match self.board.get(&cur) {
                Some(p) if *p == player => count += 1,
                _ => break,
            }
        }

        count
    }

    pub fn is_winning_move(&self, hex: Hex, player: Player) -> bool {
        for Hex(dq, dr) in Hex::axes() {
            let f = self.count_dir(hex, Hex(dq, dr), player);
            let b = self.count_dir(hex, Hex(-dq, -dr), player);

            if 1 + f + b >= self.cfg.win_distance {
                return true;
            }
        }

        false
    }

    pub fn is_within_range(&self, hex: Hex, max_dist: i32) -> bool {
        self.board
            .cells
            .keys()
            .any(|&h| h.distance(hex) <= max_dist)
            && (Hex::origin().distance(hex) <= self.cfg.size_limit)
    }

    pub fn is_game_over(&self) -> Option<Player> {
        for (&hex, &player) in self.board.cells.iter() {
            if self.is_winning_move(hex, player) {
                return Some(player);
            }
        }
        None
    }

    /// Returns Some(Vec<Hex>) of the winning line for the player, if any.
    pub fn winning_line(&self, player: Player) -> Option<Vec<Hex>> {
        for &hex in self.board.cells.keys() {
            if self.board.get(&hex) != Some(&player) {
                continue;
            }

            for &dir in Hex::axes().iter() {
                let mut line = vec![hex];

                // forward direction
                let mut cur = hex;
                loop {
                    cur = cur + dir;
                    if self.board.get(&cur) == Some(&player) {
                        line.push(cur);
                    } else {
                        break;
                    }
                }

                // backward direction
                let mut cur = hex;
                loop {
                    cur = cur - dir;
                    if self.board.get(&cur) == Some(&player) {
                        line.push(cur);
                    } else {
                        break;
                    }
                }

                if line.len() >= self.cfg.win_distance as usize {
                    // remove duplicates, sort for clarity
                    line.sort_unstable_by_key(|h| (h.0, h.1));
                    line.dedup();
                    return Some(line);
                }
            }
        }
        None
    }

    /// Applies a half-turn (placing a piece and advancing the game state) and returns an undo record.
    pub fn apply_half_turn(&mut self, hex: Hex) -> Result<Undo, Diag> {
        let undo = Undo {
            hex,
            prev_pending: self.pending,
            prev_phase: self.phase,
            prev_turn: self.turn,
            prev_zhash: self.zhash,
        };

        if !self.board.is_empty(&hex) {
            return Err(Diag::CellOccupied);
        }

        if let Some(lhs) = self.pending
            && lhs == hex
        {
            return Err(Diag::CannotPlaceTwice);
        }

        if !self.is_within_range(hex, self.cfg.view_distance) {
            return Err(Diag::TooFarAway);
        }

        self.board.place(hex, self.turn);
        self.zhash ^= zobrist::hash(hex, self.turn);

        match self.phase {
            Phase::First => {
                self.pending = Some(hex);
                self.phase = Phase::Second;
            }
            Phase::Second => {
                self.pending = None;
                self.phase = Phase::First;
                self.turn = self.turn.opponent();
            }
        }

        Ok(undo)
    }

    /// Reverts a half-turn (removes a piece and reverts the game state) using an undo record.
    pub fn undo_half_turn(&mut self, undo: Undo) {
        self.board.cells.remove(&undo.hex);
        self.zhash ^= zobrist::hash(undo.hex, undo.prev_turn);

        self.pending = undo.prev_pending;
        self.phase = undo.prev_phase;
        self.turn = undo.prev_turn;
        self.zhash = undo.prev_zhash;
    }
}
