use crate::game::{board::Board, hex::Hex, player::Player};

fn get_board_bounds(board: &Board) -> (i32, i32, i32, i32) {
    let mut min_q = 0;
    let mut max_q = 0;
    let mut min_r = 0;
    let mut max_r = 0;

    for hex in board.cells.keys() {
        min_q = min_q.min(hex.0);
        max_q = max_q.max(hex.0);
        min_r = min_r.min(hex.1);
        max_r = max_r.max(hex.1);
    }

    (min_q, max_q, min_r, max_r)
}

pub fn render_board(
    board: &Board,
    winning_line: &[Hex], // pass empty slice if no winner
    winner: Option<Player>,
) {
    use std::collections::HashSet;

    let win_set: HashSet<_> = winning_line.iter().collect();
    let (min_q, max_q, min_r, max_r) = get_board_bounds(board);

    for r in min_r..=max_r {
        // offset for pointy-top stagger
        print!("{:width$}", "", width = (max_r - r) as usize);
        for q in min_q..=max_q {
            let hex = Hex(q, r);
            let s = if win_set.contains(&hex) {
                // winning line: white
                match winner {
                    Some(Player::X) => "\x1b[39mX\x1b[0m", // white
                    Some(Player::O) => "\x1b[39mO\x1b[0m", // white
                    None => "\x1b[39m?\x1b[0m",            // white
                }
            } else if hex == Hex::origin() {
                // starting hex: bold green
                "\x1b[1;32mX\x1b[0m"
            } else if let Some(player) = board.cells.get(&hex) {
                match player {
                    Player::X => "\x1b[31mX\x1b[0m", // red
                    Player::O => "\x1b[34mO\x1b[0m", // blue
                }
            } else {
                "." // empty
            };

            print!("{} ", s);
        }

        println!();
    }
    println!();
}
