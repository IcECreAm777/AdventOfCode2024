use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileType {
    Empty,
    Corrupted
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct AStarValues {
    g: u32,
    h: u32,
}

impl AStarValues {
    pub fn new(g:u32, h: u32) -> AStarValues {
        AStarValues{g, h}
    }

    pub fn get_f(&self) -> u32 {
        self.g + self.h
    }
}

pub struct Grid {
    grid: Vec<Vec<TileType>>,
    start: (usize, usize),
    target: (usize, usize)
}

impl Grid {
    pub fn new(size: usize) -> Grid {
        let grid = vec![vec![TileType::Empty; size]; size];

        Grid { grid, start: (0, 0), target: (size-1, size-1) }
    }

    pub fn add_corrupted_space(&mut self, x: usize, y: usize) {
        self.grid[x][y] = TileType::Corrupted;
    }
    
    pub fn a_star(&self) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = vec![];
        
        let mut open_list: HashMap<(usize, usize), AStarValues> = HashMap::new();    
        let mut closed_list: HashMap<(usize, usize), AStarValues> = HashMap::new();
        let mut current = self.start;

        open_list.insert(self.start, AStarValues::new(0, 0));
    
        while current != self.target && open_list.len() > 0 {
            let values = open_list.remove(&current).unwrap();
            closed_list.insert(current, values);

            let neighbours = self.get_neighbours(current);
            for n in neighbours {
                if open_list.contains_key(&n) || closed_list.contains_key(&n) { continue; }

                let g = values.g + 1;
                let h = self.target.0.abs_diff(n.0) + self.target.1.abs_diff(n.1);

                open_list.insert(n, AStarValues::new(g, h as u32));
            }

            current = Self::get_lowest_f_value(&open_list).0;
        }

        if current != self.target { return vec![]; }

        result.push(self.target);

        while current != self.start {
            let neighbours = self.get_neighbours(current);
            let mut next: ((usize, usize), u32) = ((0, 0), 100000000);

            for n in neighbours {
                if !closed_list.contains_key(&n) { continue; }

                let new_value = closed_list[&n];
                if new_value.g < next.1 {
                    next = (n, new_value.g);
                }
            }

            current = next.0;
            result.push(current);
        }
    
        result
    }
    
    fn get_neighbours(&self, coords: (usize, usize)) -> Vec<(usize, usize)> {
        let mut result = vec![];

        if coords.0 > 0 && self.grid[coords.0-1][coords.1] != TileType::Corrupted { result.push((coords.0 - 1, coords.1)); }
        if coords.0 < self.grid.len()-1 && self.grid[coords.0+1][coords.1] != TileType::Corrupted { result.push((coords.0 + 1, coords.1)); }
        if coords.1 > 0 && self.grid[coords.0][coords.1-1] != TileType::Corrupted { result.push((coords.0, coords.1 - 1)); }
        if coords.1 < self.grid[0].len()-1 && self.grid[coords.0][coords.1+1] != TileType::Corrupted { result.push((coords.0, coords.1 + 1)); }

        result 
    }

    fn get_lowest_f_value(map: &HashMap<(usize, usize), AStarValues>) -> ((usize, usize), AStarValues) {
        let mut result = ((0, 0), AStarValues::new(0, 0));        

        for (key, value) in map.iter() {
            if result.1.get_f() == 0 {
                result = (*key, *value);
                continue;
            }

            if value.get_f() < result.1.get_f() {
                result = (*key, *value);
                continue;
            }

            if value.get_f() == result.1.get_f() {
                result = if result.1.g < value.g { result } else { (*key, *value) };
            }
        }

        result
    }
}
