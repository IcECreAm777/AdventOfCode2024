use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
enum TileState {
    Empty,
    Obstacle,
    Visited
}

pub struct Area {
    area: Vec<Vec<TileState>>,
    guard_position: (usize, usize),
    walk_direction: (i32, i32),
}

impl Area {
    pub fn new(path: &str) -> Area {
        let mut guard_position = (0, 0);
        let mut area = vec![];

        let mut i: usize = 0;
        for line in read_to_string(path).unwrap().lines() {
            let chars: Vec<char> = line.chars().collect();
            let mut new_line = vec![];

            for j in 0..chars.len() {
                let character = chars[j];
                let new_state = match character {
                    '#' => TileState::Obstacle,
                    '^' => {
                        guard_position = (i, j);
                        TileState::Visited
                    },
                    _ => TileState::Empty
                };

                new_line.push(new_state);
            }

            area.push(new_line);
            i += 1;
        }

        Area {  
            area,
            guard_position,
            walk_direction: (-1, 0)
        }
    }

    pub fn get_covered_area(&mut self) -> i32 {
        let mut counter = 1;

        while !self.is_leaving_area() {
            let next_area_index = ((self.guard_position.0 as i32 + self.walk_direction.0) as usize, (self.guard_position.1 as i32 + self.walk_direction.1) as usize);
            let next_tile = &self.area[next_area_index.0][next_area_index.1];
    
            if next_tile == &TileState::Obstacle {
                self.turn_direction();
                continue;
            }
    
            if next_tile == &TileState::Empty {
                counter += 1;
            }
    
            self.guard_position = next_area_index; 
            self.area[next_area_index.0][next_area_index.1] = TileState::Visited;
        }

        counter
    }

    fn turn_direction(&mut self) {
        let new_x = self.walk_direction.1;
        let new_y = self.walk_direction.0 * -1;
    
        self.walk_direction = (new_x, new_y);
    }
    
    fn is_leaving_area(&self) -> bool {
        return self.guard_position.0 == 0 && self.walk_direction.0 == -1 || self.guard_position.1 == 0 && self.walk_direction.1 == -1
            || self.guard_position.0 == self.area.len()-1 && self.walk_direction.0 == 1 || self.guard_position.1 == self.area[0].len()-1 && self.walk_direction.1 == 1;
    }
}