use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

const DEFAULT_WIDTH: usize = 105;
const DEFAULT_HEIGHT: usize = 23;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: SubCommands,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum NeighborMethod {
    /// The default method: all 8 surrounding cells
    Normal,
    /// The method developed by Von Neumann: 4 cardinals extended once
    VonNeumann,
}

#[derive(Subcommand, Debug)]
pub enum SubCommands {
    /// Create a grid randomly populated
    Random(Random),
    /// Use information from a file to populate a grid
    File(File),
}

#[derive(clap::Args, Debug)]
pub struct Random {
    /// Width of the grid
    #[arg(default_value_t = DEFAULT_WIDTH)]
    pub width: usize,

    /// Height of the grid
    #[arg(default_value_t = DEFAULT_HEIGHT)]
    pub height: usize,

    /// The method used to get number of neighbors
    #[arg(long, short, value_enum, default_value_t = NeighborMethod::Normal)]
    pub neighbor_method: NeighborMethod,

    /// The time slept between the next state of the board (in milliseconds) WARNING: 0 does not work!
    #[arg(long, short, default_value_t = 200)]
    pub sleep_time: u64,
}

#[derive(clap::Args, Debug)]
pub struct File {
    /// File to load from
    pub filepath: PathBuf,

    /// The method used to get number of neighbors
    #[arg(long, short, value_enum, default_value_t = NeighborMethod::Normal)]
    pub neighbor_method: NeighborMethod,

    /// The time slept between the next state of the board (in milliseconds) WARNING: 0 does not work!
    #[arg(long, short, default_value_t = 200)]
    pub sleep_time: u64,
}
