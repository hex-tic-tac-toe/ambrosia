use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    board::Cell,
    bot::Bot,
    coords::Hex,
    game::{Game, Turn},
};

use std::{fmt::Write, io::Write as _, sync::Arc};

pub fn single_game(
    gameid: u64,
    bot1: &mut Box<dyn Bot>,
    bot2: &mut Box<dyn Bot>,
    turns: u32,
    verbosity: u32,
) -> anyhow::Result<()> {
    let mut s = String::new();
    let mut game = Game::new();
    for i in 0..turns {
        if game.is_over() {
            break;
        }

        let t = game.to_move;
        if i == 0 {
            writeln!(s, "{t};{}", Hex::ZERO)?;
            if verbosity >= 3 {
                println!("\t({gameid}) turn {i}: {t} plays {}", Hex::ZERO);
            }
            game.apply(Turn::One(Hex::ZERO)).unwrap();
        }

        let turn = match game.to_move {
            Cell::X => bot1.choose(&game, Cell::X),
            Cell::O => bot2.choose(&game, Cell::O),
        };

        match turn {
            Turn::One(u) => writeln!(s, "{t};{u}")?,
            Turn::Two(u, v) => writeln!(s, "{t};{u};{v}")?,
        }

        if verbosity >= 3 {
            println!("\t({gameid}) turn {i}: {t} plays {turn:?}");
        }
        game.apply(turn).unwrap();
    }

    let result = match game.result {
        crate::game::GameResult::Winner(Cell::X) => "X",
        crate::game::GameResult::Winner(Cell::O) => "O",
        crate::game::GameResult::Draw | crate::game::GameResult::Ongoing => "D",
    };

    s = format!("# {}\n{}", result, s);

    let c = match game.result {
        crate::game::GameResult::Winner(Cell::X) => "\x1b[31m",
        crate::game::GameResult::Winner(Cell::O) => "\x1b[34m",
        crate::game::GameResult::Draw | crate::game::GameResult::Ongoing => "\x1b[37m",
    };

    if verbosity >= 2 {
        print!("{c}{result}\x1b[0m");
        std::io::stdout().flush().unwrap();
    }

    std::fs::write(
        format!("output/sets/{}-{}/{}.txt", bot1.name(), bot2.name(), gameid),
        s,
    )?;
    Ok(())
}

pub fn single_set(
    bot1: Arc<dyn Fn() -> Box<dyn Bot> + Send + Sync>,
    bot2: Arc<dyn Fn() -> Box<dyn Bot> + Send + Sync>,
    n: u32,
    turns: u32,
    verbosity: u32,
) -> anyhow::Result<()> {
    if verbosity >= 2 {
        print!("\t");
    }
    (0..n).into_par_iter().for_each(|i| {
        single_game(i as u64, &mut bot1(), &mut bot2(), turns, verbosity).unwrap();
    });
    if verbosity >= 2 {
        println!();
    }
    Ok(())
}

pub fn tournament(
    bots: &[Arc<dyn Fn() -> Box<dyn Bot> + Send + Sync>],
    n: u32,
    turns: u32,
    verbosity: u32,
) -> anyhow::Result<()> {
    println!("tournament with {} bots", bots.len());

    for i in 0..bots.len() {
        for j in 0..bots.len() {
            let ii = bots[i]();
            let jj = bots[j]();

            if verbosity >= 1 {
                println!(
                    "playing \x1b[31m{}\x1b[0m vs \x1b[34m{}\x1b[0m",
                    ii.name(),
                    jj.name()
                );
            }

            std::fs::remove_dir_all(format!("output/sets/{}-{}", ii.name(), jj.name())).ok();
            std::fs::create_dir_all(format!("output/sets/{}-{}", ii.name(), jj.name()))?;

            single_set(bots[i].clone(), bots[j].clone(), n, turns, verbosity)?;
        }
    }

    Ok(())
}
