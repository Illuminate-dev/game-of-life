pub mod ant_board;
pub mod gol_board;

pub enum Boards {
    GolBoard(gol_board::Board),
    AntBoard(ant_board::Board),
}
