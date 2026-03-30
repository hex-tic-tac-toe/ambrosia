use std::time::Instant;

use hex::{
    ai::bot::Bot,
    bots::heuristic::HeuristicBot,
    game::{config::Config, game::Game, player::Player},
    render,
};

pub fn main() {
    let cfg = Config {
        win_distance: 6,
        view_distance: 8,
        turn_limit: 300,
        size_limit: 100,
    };

    let mut game = Game::new(cfg);
    let mut bot_x = HeuristicBot::new();
    let mut bot_o = HeuristicBot::new();

    println!(
        "{}{}\x1b[0m vs {}{}\x1b[0m",
        Player::X.color(),
        bot_x.name(),
        Player::O.color(),
        bot_o.name(),
    );

    for _ in 0..game.cfg.turn_limit {
        let current_bot: &mut dyn Bot = match game.turn {
            Player::X => &mut bot_x,
            Player::O => &mut bot_o,
        };

        let i = Instant::now();
        let moves = current_bot.choose(&mut game);
        let elapsed = i.elapsed();
        println!(
            "{}{:?} ({})\x1b[0m chose {} in {:?}",
            game.turn.color(),
            game.turn,
            current_bot.name(),
            moves
                .as_ref()
                .map_or("nothing".to_owned(), |m| format!("{:?}", m)),
            elapsed
        );

        if let Some(mv) = moves {
            if let Err(err) = game.play_half_turn(mv.0) {
                println!("Invalid move {:?}: {:?}", mv.0, err);
                break;
            }

            if let Err(err) = game.play_half_turn(mv.1) {
                println!("Invalid move {:?}: {:?}", mv.1, err);
                break;
            }
            if let Some(winner) = game.is_game_over() {
                println!("{}{:?}\x1b[0m wins", winner.color(), winner);
                render::render_board(game.board.iter(), "output.png", game.winning_line(winner));
                break;
            }
        } else {
            println!("{}{:?}\x1b[0m resigns", game.turn.color(), game.turn);
            render::render_board(game.board.iter(), "output.png", vec![]);
            break;
        }
    }

    if let Some(_) = game.is_game_over() {
    } else {
        println!("Game ends in a draw");
        render::render_board(game.board.iter(), "output.png", vec![]);
    }
}
