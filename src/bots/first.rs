use crate::{
    ai::bot::Bot,
    game::{candidates::Candidates, game::Game, mv::Move},
};

pub struct FirstBot {
    candidates: Candidates,
}

impl FirstBot {
    pub fn new() -> Self {
        Self {
            candidates: Candidates::new(2),
        }
    }
}

impl Bot for FirstBot {
    fn name(&self) -> &str {
        "first"
    }

    fn choose(&mut self, game: &mut Game) -> Option<Move> {
        self.candidates.sync(game);

        let vec = self.candidates.as_vec();
        if vec.len() < 2 {
            return None;
        }

        // vec.shuffle(&mut rng());
        let mv = Move(vec[0], vec[1]);

        Some(mv)
    }
}
