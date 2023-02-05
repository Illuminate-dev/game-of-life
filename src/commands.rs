use clap::Parser;

use crate::args::{Cli, SubCommands};
use crate::boards::gol_board;
use crate::boards::{ant_board, Boards};
use crate::game::Game;
use crate::GOLError;

pub fn run() -> Result<(), GOLError> {
    let args = Cli::parse();

    match args.command {
        SubCommands::Random(args) => random(args),
        SubCommands::File(args) => from_file(args),
        SubCommands::Ant(args) => ant(args),
    }
}

fn random(args: crate::args::Random) -> Result<(), GOLError> {
    let board = gol_board::Board::random_state(args.width, args.height, args.method);

    let mut game = Game {
        board: Boards::GolBoard(board),
    };

    crate::ui::start_ui(&mut game, args.sleep_time)
}

fn from_file(args: crate::args::File) -> Result<(), GOLError> {
    let board = match gol_board::Board::load_from_file(args.filepath.to_str().unwrap()) {
        Ok(brd) => brd,
        Err(_) => return Err(GOLError::InvalidFile),
    };

    let mut game = Game {
        board: Boards::GolBoard(board),
    };

    crate::ui::start_ui(&mut game, args.sleep_time)
}

fn ant(args: crate::args::Ant) -> Result<(), GOLError> {
    let board = ant_board::Board::create_board(args.width, args.height);

    let mut game = Game {
        board: Boards::AntBoard(board),
    };

    for _ in 0..args.start {
        match &mut game.board {
            Boards::AntBoard(x) => *x = x.update(),
            _ => return Err(GOLError::UnknownError),
        }
    }

    crate::ui::start_ui(&mut game, args.interval)
}
