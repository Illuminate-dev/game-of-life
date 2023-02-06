type StateArray = Vec<Vec<bool>>;

#[derive(Clone)]
pub struct Board {
    state: StateArray,
    pub width: usize,
    pub height: usize,
    ant: (usize, usize, Direction),
}

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn rotate_right(&mut self) {
        match self {
            Direction::Up => *self = Direction::Right,
            Direction::Right => *self = Direction::Down,
            Direction::Down => *self = Direction::Left,
            Direction::Left => *self = Direction::Up,
        };
    }

    pub fn rotate_left(&mut self) {
        match self {
            Direction::Right => *self = Direction::Up,
            Direction::Down => *self = Direction::Right,
            Direction::Up => *self = Direction::Left,
            Direction::Left => *self = Direction::Down,
        };
    }
}

impl Board {
    pub fn create_board(width: usize, height: usize) -> Board {
        let state = vec![vec![false; width]; height];

        let ant = (width / 2, height / 2, Direction::Up);

        Board {
            state,
            width,
            height,
            ant,
        }
    }

    pub fn update(&self) -> Board {
        let mut new_board = self.clone();
        let is_white = !self.state[self.ant.1][self.ant.0];

        if is_white {
            new_board.ant.2.rotate_right();
        } else {
            new_board.ant.2.rotate_left();
        }

        match new_board.ant.2 {
            Direction::Up => {
                if self.ant.1 != 0 {
                    new_board.ant.1 -= 1
                } else {
                    new_board.ant.1 = self.height - 1
                }
            }
            Direction::Down => {
                if self.ant.1 < self.height - 1 {
                    new_board.ant.1 += 1
                } else {
                    new_board.ant.1 = 0
                }
            }
            Direction::Left => {
                if self.ant.0 != 0 {
                    new_board.ant.0 -= 1
                } else {
                    new_board.ant.0 = self.width - 1
                }
            }
            Direction::Right => {
                if self.ant.0 < self.width - 1 {
                    new_board.ant.0 += 1
                } else {
                    new_board.ant.0 = 0
                }
            }
        };

        new_board.state[self.ant.1][self.ant.0] = is_white;

        new_board
    }

    pub fn render(&self) -> String {
        let mut string = String::new();
        let divider = "-".repeat(self.width + 2);
        string.push_str(&divider);
        string.push('\n');
        for (y, line) in self.state.iter().enumerate() {
            string.push('|');
            for (x, cell) in line.iter().enumerate() {
                if self.ant.0 == x && self.ant.1 == y {
                    string.push('X');
                } else if *cell {
                    string.push('#');
                } else {
                    string.push(' ');
                }
            }
            string.push('|');
            string.push('\n');
        }
        string.push_str(&divider);

        string
    }
}
