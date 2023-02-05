pub mod args;
pub mod board;
pub mod commands;
mod inputs;
pub mod ui;

#[derive(Debug)]
pub enum GOLError {}

pub struct Config {
    pub sleep_time: u64,
    pub neighbor_method: args::GOLMethod,
}
