use rand::prelude::*;

type StateArray = Vec<Vec<bool>>;

#[derive(Debug)]
pub struct Board {
    state: StateArray,
    width: usize,
    height: usize,
}

impl Board {
    pub fn dead_state(width: usize, height: usize) -> Board {
        let state = vec![vec![false; width]; height];

        Board {
            state,
            width,
            height,
        }
    }

    pub fn random_state(width: usize, height: usize) -> Board {
        let mut board = Self::dead_state(width, height);

        for i in board.state.iter_mut().flatten() {
            *i = random();
        }

        board
    }

    pub fn render(&self) -> String {
        let mut string = String::new();
        let divider = "-".repeat(self.width + 2);
        string.push_str(&divider);
        string.push('\n');
        for y in &self.state {
            string.push('|');
            for x in y {
                if *x {
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

    pub fn next_state(&self) -> Board {
        let mut new_state = self.state.clone();
        for (y, line) in self.state.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                let neighbors = self.calculate_num_neighbors(x, y);
                if *cell {
                    if neighbors < 2 || neighbors > 3 {
                        new_state[y][x] = false;
                    }
                } else if neighbors == 3 {
                    new_state[y][x] = true;
                }
            }
        }

        Board {
            state: new_state,
            width: self.width,
            height: self.height,
        }
    }

    fn calculate_num_neighbors(&self, x: usize, y: usize) -> u8 {
        let left = x != 0;
        let right = x < self.width - 1;
        let up = y != 0;
        let down = y < self.height - 1;

        let mut counter = 0;

        if left && self.state[y][x - 1] {
            counter += 1;
        }

        if right && self.state[y][x + 1] {
            counter += 1;
        }

        if up && self.state[y - 1][x] {
            counter += 1;
        }

        if down && self.state[y + 1][x] {
            counter += 1;
        }

        if left && up && self.state[y - 1][x - 1] {
            counter += 1;
        }

        if left && down && self.state[y + 1][x - 1] {
            counter += 1;
        }

        if right && up && self.state[y - 1][x + 1] {
            counter += 1;
        }

        if right && down && self.state[y + 1][x + 1] {
            counter += 1;
        }

        counter
    }

    pub fn load_from_file(filename: &str) -> std::io::Result<Board> {
        let file = std::fs::read_to_string(filename)?;
        let state = file.lines().fold(Vec::new(), |mut vec, line: &str| {
            vec.push(line.chars().fold(Vec::new(), |mut acc, i| {
                if i == '0' {
                    acc.push(false);
                } else {
                    acc.push(true);
                }
                acc
            }));
            vec
        });

        Ok(Board {
            state,
            width: file.lines().collect::<Vec<&str>>()[0].len(),
            height: file.lines().collect::<Vec<&str>>().len(),
        })
    }
}

#[cfg(test)]
mod tests {

    #[test]
    pub fn test_state() {
        let init_state = vec![vec![false; 5]; 5];

        let board = super::Board {
            state: init_state,
            width: 5,
            height: 5,
        };

        let expected_state = vec![vec![false; 5]; 5];

        let next_state = board.next_state();

        assert_eq!(expected_state, next_state.state);
    }

    #[test]
    pub fn test_birth_state() {
        let init_state = vec![
            vec![false, false, true],
            vec![false, true, true],
            vec![false, false, false],
        ];

        let board = super::Board {
            state: init_state,
            width: 3,
            height: 3,
        };

        let expected_state = vec![
            vec![false, true, true],
            vec![false, true, true],
            vec![false, false, false],
        ];

        let next_state = board.next_state();

        assert_eq!(expected_state, next_state.state);
    }
}
