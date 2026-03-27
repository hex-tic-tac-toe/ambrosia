use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct Hex {
    pub q: i32,
    pub r: i32,
}

impl Hex {
    pub const ZERO: Self = Self { q: 0, r: 0 };
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    pub const DIRECTIONS: [Hex; 6] = [
        Hex { q: 1, r: 0 },
        Hex { q: 1, r: -1 },
        Hex { q: 0, r: -1 },
        Hex { q: -1, r: 0 },
        Hex { q: -1, r: 1 },
        Hex { q: 0, r: 1 },
    ];

    pub const AXES: [(Hex, Hex); 3] = [
        (Hex { q: 1, r: 0 }, Hex { q: -1, r: 0 }),  // horizontal
        (Hex { q: 0, r: 1 }, Hex { q: 0, r: -1 }),  // diagonal "\"
        (Hex { q: 1, r: -1 }, Hex { q: -1, r: 1 }), // diagonal "/"
    ];

    pub fn distance(&self, other: &Self) -> i32 {
        let dq = self.q - other.q;
        let dr = self.r - other.r;
        let ds = dq + dr;
        (dq.abs() + dr.abs() + ds.abs()) / 2
    }
}

impl std::ops::Add for Hex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            q: self.q + other.q,
            r: self.r + other.r,
        }
    }
}

impl std::ops::Sub for Hex {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            q: self.q - other.q,
            r: self.r - other.r,
        }
    }
}

impl std::ops::Mul<i32> for Hex {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        Self {
            q: self.q * other,
            r: self.r * other,
        }
    }
}

impl Display for Hex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // return xyz coordinates
        write!(f, "({}, {}, {})", self.q, self.r, -self.q - self.r)
    }
}

