use std::thread;
use std::time::Duration;

use clap::Parser;

use crate::args::{Cli, NeighborMethod, SubCommands};
use crate::board;
use crate::GOLError;

pub fn run() -> Result<(), GOLError> {
    let args = Cli::parse();

    match args.command {
        SubCommands::Random(args) => random(args),
        SubCommands::FromFile(args) => from_file(args),
    }
}

fn random(args: crate::args::Random) -> Result<(), GOLError> {
    let neighbor_function = match args.neighbor_method {
        NeighborMethod::Normal => board::Board::next_state,
        NeighborMethod::VonNeumann => board::Board::next_state_neumann,
    };

    let mut board = board::Board::random_state(args.width, args.height);

    println!("{}", board.render());

    loop {
        board = neighbor_function(&board);
        println!("{}", board.render());
        thread::sleep(Duration::from_millis(args.sleep_time));
    }
}

fn from_file(args: crate::args::FromFile) -> Result<(), GOLError> {
    let neighbor_function = match args.neighbor_method {
        NeighborMethod::Normal => board::Board::next_state,
        NeighborMethod::VonNeumann => board::Board::next_state_neumann,
    };

    let mut board = board::Board::load_from_file(args.filepath.to_str().unwrap()).unwrap();

    println!("{}", board.render());

    loop {
        board = neighbor_function(&board);
        println!("{}", board.render());
        thread::sleep(Duration::from_millis(args.sleep_time));
    }
}
