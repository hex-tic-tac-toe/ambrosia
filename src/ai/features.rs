use crate::{
    ai::feature::Feature,
    game::{game::Game, hex::Hex, player::Player},
};

impl Game {
    pub fn my_cells(&self) -> impl Iterator<Item = Hex> + '_ {
        let p = self.turn;
        self.board
            .cells
            .iter()
            .filter(move |(_, v)| **v == p)
            .map(|(&k, _)| k)
    }

    pub fn opp_cells(&self) -> impl Iterator<Item = Hex> + '_ {
        let p = self.turn.opponent();
        self.board
            .cells
            .iter()
            .filter(move |(_, v)| **v == p)
            .map(|(&k, _)| k)
    }

    pub fn run_len(&self, start: Hex, dir: Hex, player: Player) -> i32 {
        self.count_dir(start, dir, player)
    }

    pub fn run_pair(&self, hex: Hex, dir: Hex, player: Player) -> (i32, i32) {
        let fwd = self.run_len(hex, dir, player);
        let bwd = self.run_len(hex, Hex(-dir.0, -dir.1), player);
        (fwd, bwd)
    }

    pub fn line_len(&self, hex: Hex, dir: Hex, player: Player) -> i32 {
        let (f, b) = self.run_pair(hex, dir, player);
        1 + f + b
    }

    pub fn open_ends(&self, hex: Hex, dir: Hex, player: Player) -> i32 {
        let (f, b) = self.run_pair(hex, dir, player);
        let front = hex + Hex(dir.0 * (f + 1), dir.1 * (f + 1));
        let back = hex + Hex(-dir.0 * (b + 1), -dir.1 * (b + 1));
        let front_open =
            self.board.is_empty(&front) && Hex::origin().distance(front) <= self.cfg.size_limit;
        let back_open =
            self.board.is_empty(&back) && Hex::origin().distance(back) <= self.cfg.size_limit;
        front_open as i32 + back_open as i32
    }
}

pub struct LongestRun;
impl Feature<Game> for LongestRun {
    fn name(&self) -> &'static str {
        "longest_run"
    }

    fn score(&self, game: &Game) -> f64 {
        let player = game.turn;
        game.my_cells()
            .flat_map(|h| Hex::axes().into_iter().map(move |d| (h, d)))
            .map(|(h, d)| game.line_len(h, d, player))
            .max()
            .unwrap_or(0) as f64
    }
}

pub struct ThreatScore;

impl Feature<Game> for ThreatScore {
    fn name(&self) -> &'static str {
        "threat_score"
    }

    fn score(&self, game: &Game) -> f64 {
        let player = game.turn;
        let win = game.cfg.win_distance;
        let mut total = 0.0f64;

        for hex in game.my_cells() {
            for dir in Hex::axes() {
                // only score from the "start" of each run to avoid double-counting
                let bwd = Hex(-dir.0, -dir.1);
                if game.board.get(&(hex + bwd)) == Some(&player) {
                    continue;
                }

                let (fwd_len, _) = game.run_pair(hex, dir, player);
                let len = 1 + fwd_len;
                let ends = game.open_ends(hex, dir, player);

                // skip runs that can never reach win_distance
                if len + ends * (win - len) < win {
                    continue;
                }

                total += (len * len) as f64 * (1 + ends) as f64;
            }
        }

        total
    }
}

pub struct OpenThreats(pub i32);
impl Feature<Game> for OpenThreats {
    fn name(&self) -> &'static str {
        "open_threats"
    }

    fn score(&self, game: &Game) -> f64 {
        let player = game.turn;
        let win = game.cfg.win_distance;
        let min_len = win - self.0;
        let mut count = 0.0f64;

        for hex in game.my_cells() {
            for dir in Hex::axes() {
                let bwd = Hex(-dir.0, -dir.1);
                if game.board.get(&(hex + bwd)) == Some(&player) {
                    continue; // not the start
                }

                let (fwd_len, _) = game.run_pair(hex, dir, player);
                let len = 1 + fwd_len;
                let ends = game.open_ends(hex, dir, player);

                if len >= min_len && ends >= 1 {
                    count += 1.0;
                }
            }
        }

        count
    }
}

pub struct DoubleThreats;

impl Feature<Game> for DoubleThreats {
    fn name(&self) -> &'static str {
        "double_threats"
    }

    fn score(&self, game: &Game) -> f64 {
        let player = game.turn;
        let win = game.cfg.win_distance;
        let mut count = 0.0f64;

        // check every empty candidate cell
        for (&hex, _) in game.board.cells.iter() {
            for dir in Hex::axes() {
                // look at nearby empty cells around occupied ones
                for dist in 1..=2i32 {
                    let candidate = hex + Hex(dir.0 * dist, dir.1 * dist);
                    if !game.board.is_empty(&candidate) {
                        continue;
                    }
                    if Hex::origin().distance(candidate) > game.cfg.size_limit {
                        continue;
                    }

                    // count axes on which placing here would create a near-win
                    let threatening_axes = Hex::axes()
                        .into_iter()
                        .filter(|&d| {
                            let (f, b) = game.run_pair(candidate, d, player);
                            1 + f + b >= win - 1
                        })
                        .count();

                    if threatening_axes >= 2 {
                        count += 1.0;
                    }
                }
            }
        }

        count
    }
}

pub struct GapThreats;

impl Feature<Game> for GapThreats {
    fn name(&self) -> &'static str {
        "gap_threats"
    }

    fn score(&self, game: &Game) -> f64 {
        let player = game.turn;
        let win = game.cfg.win_distance;
        let mut count = 0.0f64;

        for hex in game.my_cells() {
            for dir in Hex::axes() {
                // look one cell ahead for a gap
                let gap = hex + dir;
                if !game.board.is_empty(&gap) {
                    continue;
                }

                // look past the gap
                let far = gap + dir;
                if game.board.get(&far) != Some(&player) {
                    continue;
                }

                // measure the full "virtual" run if gap were filled:
                // pieces behind hex + 1 (hex) + 1 (gap) + pieces from far forward
                let (_, behind) = game.run_pair(hex, dir, player);
                let (ahead, _) = game.run_pair(far, dir, player);
                let virtual_len = 1 + behind + 1 + 1 + ahead; // hex + gap + far + extensions

                if virtual_len >= win - 1 {
                    count += 1.0;
                }
            }
        }

        count
    }
}

pub struct OpponentThreat;

impl Feature<Game> for OpponentThreat {
    fn name(&self) -> &'static str {
        "opponent_threat"
    }

    fn score(&self, game: &Game) -> f64 {
        let opp = game.turn.opponent();
        let win = game.cfg.win_distance;
        let mut total = 0.0f64;

        for hex in game.opp_cells() {
            for dir in Hex::axes() {
                let bwd = Hex(-dir.0, -dir.1);
                if game.board.get(&(hex + bwd)) == Some(&opp) {
                    continue;
                }

                let (fwd_len, _) = game.run_pair(hex, dir, opp);
                let len = 1 + fwd_len;
                let ends = game.open_ends(hex, dir, opp);

                if len + ends * (win - len) < win {
                    continue;
                }

                total += (len * len) as f64 * (1 + ends) as f64;
            }
        }

        total
    }
}

pub struct LargestCluster;

impl Feature<Game> for LargestCluster {
    fn name(&self) -> &'static str {
        "largest_cluster"
    }

    fn score(&self, game: &Game) -> f64 {
        let player = game.turn;
        let my: Vec<Hex> = game.my_cells().collect();
        if my.is_empty() {
            return 0.0;
        }

        let mut visited = std::collections::HashSet::new();
        let mut best = 0usize;

        for &start in &my {
            if visited.contains(&start) {
                continue;
            }
            // BFS over direct hex neighbours
            let mut queue = vec![start];
            let mut size = 0;
            while let Some(h) = queue.pop() {
                if !visited.insert(h) {
                    continue;
                }
                size += 1;
                for dir in Hex::axes() {
                    for &step in &[dir, Hex(-dir.0, -dir.1)] {
                        let nb = h + step;
                        if game.board.get(&nb) == Some(&player) && !visited.contains(&nb) {
                            queue.push(nb);
                        }
                    }
                }
            }
            best = best.max(size);
        }

        best as f64
    }
}

pub struct IsolatedPieces;

impl Feature<Game> for IsolatedPieces {
    fn name(&self) -> &'static str {
        "isolated_pieces"
    }

    fn score(&self, game: &Game) -> f64 {
        let player = game.turn;
        game.my_cells()
            .filter(|&h| {
                !Hex::axes().into_iter().any(|dir| {
                    (1..=2).any(|d| {
                        let nb = h + Hex(dir.0 * d, dir.1 * d);
                        game.board.get(&nb) == Some(&player)
                    }) || (1..=2).any(|d| {
                        let nb = h + Hex(-dir.0 * d, -dir.1 * d);
                        game.board.get(&nb) == Some(&player)
                    })
                })
            })
            .count() as f64
    }
}

pub struct CentreProximity;

impl Feature<Game> for CentreProximity {
    fn name(&self) -> &'static str {
        "centre_proximity"
    }

    fn score(&self, game: &Game) -> f64 {
        game.my_cells()
            .map(|h| 1.0 / (1.0 + Hex::origin().distance(h) as f64))
            .sum()
    }
}
