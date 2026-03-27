use crate::{
    board::Cell,
    game::{Game, Turn},
};

pub trait Bot: Sync + Send {
    fn name(&self) -> &str;
    fn choose(&mut self, game: &Game, player: Cell) -> Turn;
}
