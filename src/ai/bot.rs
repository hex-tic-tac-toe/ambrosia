use crate::game::{game::Game, mv::Move};

pub trait Bot {
    fn name(&self) -> &str;
    /// Returns a suggested move for the current player, or `None` if we resign.
    /// The caller must ensure that `game` is equal before and after calling this function.
    fn choose(&mut self, game: &mut Game) -> Option<Move>;
}
