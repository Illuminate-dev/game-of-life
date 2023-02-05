pub mod args;
pub mod commands;
mod game;
mod inputs;
pub mod ui;
pub mod boards;

#[derive(Debug)]
pub enum GOLError {
    InvalidFile
}
