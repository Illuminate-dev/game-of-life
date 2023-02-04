use std::{thread, time::Duration};

use game_of_life::board::Board;

fn main() {
    let mut board = Board::random_state(200, 200);

    println!("{}", board.render());

    loop {
        board = board.next_state_neumann();
        println!("{}", board.render());
        thread::sleep(Duration::from_millis(100));
    }
}
