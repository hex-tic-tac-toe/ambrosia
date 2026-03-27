use crate::{
    board::Cell,
    bot::Bot,
    game::{Game, Turn},
};

pub struct FirstBot;

impl FirstBot {
    pub fn new() -> Self {
        Self
    }
}

impl Bot for FirstBot {
    fn name(&self) -> &str {
        "first"
    }

    fn choose(&mut self, game: &Game, player: Cell) -> Turn {
        let candidates = game.board.legal_placements().collect::<Vec<_>>();
        assert!(candidates.len() >= 2);

        // select winning moves first
        if let Some(&win_hex) = candidates
            .iter()
            .find(|&&h| game.board.is_winning_move(h, player))
        {
            // Pick any other legal cell as the second placement.
            let second = candidates.iter().find(|&&h| h != win_hex).copied().unwrap();
            return Turn::Two(win_hex, second);
        }

        Turn::Two(candidates[0], candidates[1])
    }
}
