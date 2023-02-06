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
pub enum GOLMethod {
    /// The default method: normal GOL with neighbors being all 8 surrounding cells
    Normal,
    /// Von Neumann's method: GOL with neighbors being 4 cardinals extended once
    VonNeumann,
    /// Day and Night: B3678/S34678
    Dan,
}

#[derive(Subcommand, Debug)]
pub enum SubCommands {
    /// Create a grid randomly populated
    Random(Random),
    /// Use information from a file to populate a grid
    File(File),
    /// Run Langton's Ant
    Ant(Ant),
}

#[derive(clap::Args, Debug)]
pub struct Random {
    /// Width of the grid
    #[arg(default_value_t = DEFAULT_WIDTH)]
    pub width: usize,

    /// Height of the grid
    #[arg(default_value_t = DEFAULT_HEIGHT)]
    pub height: usize,

    /// The method used for the simulation
    #[arg(long, short, value_enum, default_value_t = GOLMethod::Normal)]
    pub method: GOLMethod,

    /// The time slept between updating the board (in milliseconds)
    #[arg(long, short, default_value_t = 200)]
    pub sleep_time: u64,
}

#[derive(clap::Args, Debug)]
pub struct File {
    /// File to load from
    pub filepath: PathBuf,

    /// The method used for the simulation
    #[arg(long, short, value_enum, default_value_t = GOLMethod::Normal)]
    pub method: GOLMethod,

    /// The time slept between updating the board (in milliseconds)
    #[arg(long, short, default_value_t = 200)]
    pub sleep_time: u64,
}

// Add argument for Langtons' ant

#[derive(clap::Args, Debug)]
pub struct Ant {
    /// Width of the grid
    #[arg(default_value_t = DEFAULT_WIDTH)]
    pub width: usize,

    /// Height of the grid
    #[arg(default_value_t = DEFAULT_HEIGHT)]
    pub height: usize,

    /// The time slept between updating the board (in milliseconds)
    #[arg(long, short, default_value_t = 200)]
    pub interval: u64,

    /// The step to start at
    #[arg(long, short, default_value_t = 0)]
    pub start: u64,
}
