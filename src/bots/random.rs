use crate::{
    ai::{bot::Bot, movegen},
    game::{game::Game, mv::Move},
};
use rand::rng;
use rand::seq::SliceRandom;
pub struct RandomBot;

impl Bot for RandomBot {
    fn name(&self) -> &str {
        "random"
    }

    fn choose(&self, game: &mut Game) -> Option<Move> {
        let mut candidates = movegen::generate_candidates(game, 2);

        let mut rng = rng();
        candidates.shuffle(&mut rng);

        if candidates.len() < 2 {
            return None;
        }

        Some(Move(candidates[0], candidates[1]))
    }
}
