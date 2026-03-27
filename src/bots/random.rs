use rand::Rng;

use crate::{
    board::Cell,
    bot::Bot,
    game::{Game, Turn},
};

pub struct RandomBot;

impl RandomBot {
    pub fn new() -> Self {
        Self
    }
}

impl Bot for RandomBot {
    fn name(&self) -> &str {
        "random"
    }

    fn choose(&mut self, game: &Game, player: Cell) -> Turn {
        let mut candidates = game.board.legal_placements().collect::<Vec<_>>();
        assert!(candidates.len() >= 2);

        // select winning moves first
        if let Some(&win_hex ) = candidates
            .iter()
            .find(|&&h | game.board.is_winning_move(h, player))
        {
            // Pick any other legal cell as the second placement.
            let second = candidates
                .iter()
                .find(|&&h | h != win_hex)
                .copied()
                .unwrap();
            return Turn::Two(win_hex, second);
        }

        let mut r = rand::rng();

        let i = r.next_u64() % candidates.len() as u64;
        candidates.swap(0, i as usize);
        let j = r.next_u64() % (candidates.len() - 1) as u64;
        candidates.swap(1, j as usize);

        Turn::Two(candidates[0], candidates[1])
    }
}
