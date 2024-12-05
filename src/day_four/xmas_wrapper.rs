use std::fs::read_to_string;

pub struct XMasWrapper {
    lines: Vec<String>
}

impl XMasWrapper {
    pub fn new() -> XMasWrapper {
        XMasWrapper {
            lines: vec![]
        }
    }
    
    pub fn initialize_from_file(&mut self, path: &str) {
        for line in read_to_string(path).unwrap().lines() {
            let parsed = String::from(line);
            self.lines.push(parsed);
        }
    }

    pub fn find_xmas(&self) -> i64 {
        let mut result = 0;

        for i in 0..self.lines.len() {
            for j in 0..self.lines[i].len() {
                let character = self.lines[i].chars().nth(j).unwrap();
                if character == 'X' {
                    result += self.lookup(i, j);
                }
            }
        }

        result
    }

    fn lookup(&self, i: usize, j: usize) -> i64 {
        let mut result = 0;

        let right_possible = j + 3 < self.lines.len();
        let left_possible = j >= 3;

        if left_possible {
            result += self.look_in_direction(i, j, 0, -1);
        }

        if right_possible {
            result += self.look_in_direction(i, j, 0, 1);
        }

        if i >= 3 {
            result += self.look_in_direction(i, j, -1, 0);

            if left_possible {
                result += self.look_in_direction(i, j, -1, -1);
            }

            if right_possible {
                result += self.look_in_direction(i, j, -1, 1);
            }
        }

        if i + 3 < self.lines.len() {
            result += self.look_in_direction(i, j, 1, 0);

            if left_possible {
                result += self.look_in_direction(i, j, 1, -1);
            }

            if right_possible {
                result += self.look_in_direction(i, j, 1, 1);
            }
        }

        result
    }

    fn look_in_direction(&self, i: usize, j: usize, to_i: i64, to_j: i64) -> i64 {
        if self.lines[(i as i64 + to_i) as usize].chars().nth((j as i64 + to_j) as usize).unwrap() != 'M' {
            return 0;
        }

        if self.lines[(i as i64 + to_i * 2) as usize].chars().nth((j as i64 + to_j * 2) as usize).unwrap() != 'A' {
            return 0;
        }

        if self.lines[(i as i64 + to_i * 3) as usize].chars().nth((j as i64 + to_j * 3) as usize).unwrap() != 'S' {
            return 0;
        }

        1
    }
}
