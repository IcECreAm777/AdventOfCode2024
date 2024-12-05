use std::fs::read_to_string;

pub struct CrossedMasWrapper {
    lines: Vec<String>
}

impl CrossedMasWrapper {
    pub fn new(path: &str) -> CrossedMasWrapper {
        let mut result = CrossedMasWrapper {
            lines: vec![]
        };

        for line in read_to_string(path).unwrap().lines() {
            let parsed = String::from(line);
            result.lines.push(parsed);
        }

        result
    }

    pub fn find_crossed_mas(&self) -> i64 {
        let mut result = 0;

        for i in 1..self.lines.len()-1 {
            for j in 1..self.lines[i].len()-1 {
                let character = self.lines[i].chars().nth(j).unwrap();
                if character == 'A' {
                    result += self.lookup(i, j);
                }
            }
        }

        result
    }

    fn lookup(&self, i: usize, j:usize) -> i64 {
        let mut left_to_right = false;
        let mut right_to_left = false;

        if self.lines[i-1].chars().nth(j-1).unwrap() == 'M' {
            if self.lines[i+1].chars().nth(j+1).unwrap() == 'S' {
                left_to_right = true;
            }
        }

        if self.lines[i-1].chars().nth(j-1).unwrap() == 'S' {
            if self.lines[i+1].chars().nth(j+1).unwrap() == 'M' {
                left_to_right = true;
            }
        }

        if self.lines[i-1].chars().nth(j+1).unwrap() == 'M' {
            if self.lines[i+1].chars().nth(j-1).unwrap() == 'S' {
                right_to_left = true;
            }
        }

        if self.lines[i-1].chars().nth(j+1).unwrap() == 'S' {
            if self.lines[i+1].chars().nth(j-1).unwrap() == 'M' {
                right_to_left = true;
            }
        }

        if right_to_left && left_to_right { 1 } else { 0 }
    }
}
