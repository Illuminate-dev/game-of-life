use std::{thread, time::Duration};

use game_of_life::board::Board;

fn main() {
    let mut board = Board::load_from_file("examples/beacon.txt").unwrap();

    println!("{}", board.render());

    loop {
        board = board.next_state();
        println!("{}", board.render());
        thread::sleep(Duration::from_millis(100));
    }
}
