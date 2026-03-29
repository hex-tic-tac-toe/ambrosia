use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Hex(pub i32, pub i32);

impl Hex {
    pub const fn new(q: i32, r: i32) -> Self {
        Self(q, r)
    }

    pub const fn origin() -> Self {
        Self(0, 0)
    }

    pub fn distance(self, rhs: Hex) -> i32 {
        let dq = self.0 - rhs.0;
        let dr = self.1 - rhs.1;
        let ds = (-self.0 - self.1) - (-rhs.0 - rhs.1);
        dq.abs().max(dr.abs()).max(ds.abs())
    }

    pub const fn axes() -> [Self; 3] {
        [Self::new(1, 0), Self::new(0, 1), Self::new(1, -1)]
    }

    pub const fn directions() -> [Self; 6] {
        [
            Self::new(1, 0),
            Self::new(1, -1),
            Self::new(0, -1),
            Self::new(-1, 0),
            Self::new(-1, 1),
            Self::new(0, 1),
        ]
    }
}

impl Add for Hex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Hex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}
