type StateArray = Vec<Vec<bool>>;

#[derive(Clone)]
pub struct Board {
    state: StateArray,
    pub width: usize,
    pub height: usize,
    ant: (usize, usize),
}

impl Board {
    pub fn create_board(width: usize, height: usize) -> Board {
        let state = vec![vec![false; width]; height];

        let ant = (width / 2, height / 2);

        Board {
            state,
            width,
            height,
            ant,
        }
    }

    pub fn update(&self) -> Board {
        self.clone()
    }

    pub fn render(&self) -> String {
        let mut string = String::new();
        let divider = "-".repeat(self.width + 2);
        string.push_str(&divider);
        string.push('\n');
        for (y, line) in self.state.iter().enumerate() {
            string.push('|');
            for (x, cell) in line.iter().enumerate() {
                if *cell {
                    string.push('#');
                } else if self.ant.0 == x && self.ant.1 == y {
                    string.push('X');
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
