#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn opponent(&self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            Self::X => "\x1b[31m",
            Self::O => "\x1b[34m",
        }
    }
}
