use std::fs::read_to_string;

#[derive(Debug, PartialEq, Clone, Copy)]
enum TileState {
    Obstacle,
    Up,
    Right,
    Down,
    Left
}

pub struct ComplexArea {
    area: Vec<Vec<Vec<TileState>>>,
    guard_position: (usize, usize),
    results: Vec<(usize, usize)>
}

impl ComplexArea {
    pub fn new(path: &str) -> ComplexArea {
        let mut guard_position = (0, 0);
        let mut area = vec![];

        let mut i: usize = 0;
        for line in read_to_string(path).unwrap().lines() {
            let chars: Vec<char> = line.chars().collect();
            let mut new_line = vec![];

            for j in 0..chars.len() {
                let character = chars[j];
                match character {
                    '#' => { new_line.push(vec![TileState::Obstacle]);},
                    '^' => { 
                        new_line.push(vec![TileState::Up]);
                        guard_position = (i, j);
                    },
                    _ => { new_line.push(vec![]); }
                };
            }

            area.push(new_line);
            i += 1;
        }

        ComplexArea {  
            area,
            guard_position,
            results: vec![]
        }
    }

    pub fn get_num_possible_loops(&mut self) -> i32 {
        let mut walk_direction = (-1, 0);

        while !Self::is_leaving_area(&self.area, self.guard_position, walk_direction) {
            let next_area_index = ((self.guard_position.0 as i32 + walk_direction.0) as usize, (self.guard_position.1 as i32 + walk_direction.1) as usize);
            let next_tiles = &self.area[next_area_index.0][next_area_index.1];

            if next_tiles.len() > 0 && next_tiles[0] == TileState::Obstacle {
                walk_direction = Self::get_turned_direction(walk_direction);

                let new_direction = Self::get_type_by_direction(walk_direction);
                Self::update_type_at_position(&mut self.area, new_direction, self.guard_position);

                continue;
            }

            if !self.results.contains(&next_area_index) && next_tiles.len() == 0 {
                let mut copy = self.area.clone();
                if Self::find_way_in_area(&mut copy, self.guard_position, walk_direction) {
                    self.results.push(next_area_index);
                }
            }

            let target_state = Self::get_type_by_direction(walk_direction);
            Self::update_type_at_position(&mut self.area, target_state, next_area_index);
            self.guard_position = next_area_index;
        }

        self.results.len() as i32
    }

    fn find_way_in_area(area: &mut Vec<Vec<Vec<TileState>>>, start_pos: (usize, usize), start_direction: (i32, i32)) -> bool {
        if Self::is_leaving_area(&area, start_pos, start_direction) {
            return false;
        }

        let obstacle_index = ((start_pos.0 as i32 + start_direction.0) as usize, (start_pos.1 as i32 + start_direction.1) as usize);
        area[obstacle_index.0][obstacle_index.1] = vec![TileState::Obstacle];

        let mut guard_pos = start_pos;
        let mut direction = start_direction;

        while !Self::is_leaving_area(&area, guard_pos, direction) {
            let next_area_index = ((guard_pos.0 as i32 + direction.0) as usize, (guard_pos.1 as i32 + direction.1) as usize);
            let next_tiles = &area[next_area_index.0][next_area_index.1];
            let target_state = Self::get_type_by_direction(direction);

            if next_tiles.len() > 0 {
                if next_tiles.contains(&target_state) {
                    return true;
                }

                if next_tiles[0] == TileState::Obstacle {
                    direction = Self::get_turned_direction(direction);

                    let new_state = Self::get_type_by_direction(direction);
                    Self::update_type_at_position(area, new_state, guard_pos);

                    continue;
                }
            }

            Self::update_type_at_position(area, target_state, next_area_index);
            guard_pos = next_area_index;
        }

        false
    }
    
    fn is_leaving_area(area: &Vec<Vec<Vec<TileState>>>, position: (usize, usize), direction: (i32, i32)) -> bool {
        return position.0 == 0 && direction.0 == -1 || position.1 == 0 && direction.1 == -1
            || position.0 == area.len()-1 && direction.0 == 1 || position.1 == area[0].len()-1 && direction.1 == 1;
    }

    fn get_type_by_direction(direction: (i32, i32)) -> TileState {
        match direction {
            (-1, 0) => TileState::Up,
            (0, 1) => TileState::Right,
            (1, 0) => TileState::Down,
            (0, -1) => TileState::Left,
            _ => TileState::Obstacle
        }
    }

    fn update_type_at_position(area: &mut Vec<Vec<Vec<TileState>>>, target_state: TileState, index: (usize, usize)) {
        if !area[index.0][index.1].contains(&target_state) {
            area[index.0][index.1].push(target_state);
        }
    }

    fn get_turned_direction(direction: (i32, i32)) -> (i32, i32) {
        let new_x = direction.1;
        let new_y = direction.0 * -1;

        return (new_x, new_y);
    }
}
