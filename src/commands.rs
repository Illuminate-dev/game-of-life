
use clap::Parser;

use crate::args::{Cli, SubCommands};
use crate::GOLError;
use crate::{board, Config};

pub fn run() -> Result<(), GOLError> {
    let args = Cli::parse();

    match args.command {
        SubCommands::Random(args) => random(args),
        SubCommands::File(args) => from_file(args),
    }
}

fn random(args: crate::args::Random) -> Result<(), GOLError> {

    let mut board = board::Board::random_state(args.width, args.height);

    let config = Config {
        sleep_time: args.sleep_time,
        neighbor_method: args.neighbor_method,
    };

    crate::ui::start_ui(&mut board, config)
}

fn from_file(args: crate::args::File) -> Result<(), GOLError> {
    let mut board = board::Board::load_from_file(args.filepath.to_str().unwrap()).unwrap();

    let config = Config {
        sleep_time: args.sleep_time,
        neighbor_method: args.neighbor_method,
    };

    crate::ui::start_ui(&mut board, config)
}

