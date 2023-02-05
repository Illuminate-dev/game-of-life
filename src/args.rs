use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

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
    FromFile(FromFile),
}

#[derive(clap::Args, Debug)]
pub struct Random {
    /// Width of the grid
    #[arg(default_value_t = 50)]
    pub width: usize,

    /// Height of the grid
    #[arg(default_value_t = 50)]
    pub height: usize,

    /// The method used to get number of neighbors
    #[arg(long, short, value_enum, default_value_t = NeighborMethod::Normal)]
    pub neighbor_method: NeighborMethod,

    /// The time slept between the next state of the board
    #[arg(long, short, default_value_t = 200)]
    pub sleep_time: u64,
}

#[derive(clap::Args, Debug)]
pub struct FromFile {
    /// File to load from
    pub filepath: PathBuf,

    /// The method used to get number of neighbors
    #[arg(long, short, value_enum, default_value_t = NeighborMethod::Normal)]
    pub neighbor_method: NeighborMethod,

    /// The time slept between the next state of the board
    #[arg(long, short, default_value_t = 200)]
    pub sleep_time: u64,
}
