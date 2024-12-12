use std::{collections::HashMap, fs::read_to_string};

pub fn run_day_12() {
    let path = "src\\day_twelve\\input.txt";
    
    let mut plot: Vec<Vec<char>> = vec![];

    for line in read_to_string(path).unwrap().lines() {
        let next_row: Vec<char> = line.chars().collect();
        plot.push(next_row);   
    }

    let mut regions = vec![];

    for i in 0..plot.len() {
        for j in 0..plot[i].len() {
            let current = plot[i][j];
            if current == '.' { continue; }

            regions.push(find_region(plot.as_mut(), (i, j)));
        }
    }

    let mut task_one_result = 0;
    for region in regions {
        task_one_result += region.get_cost();
    }

    println!("\ttask one result: {}", task_one_result);
}

fn find_region(area: &mut Vec<Vec<char>>, start_index: (usize, usize)) -> Region {
    let mut to_check = vec![start_index];
    let mut coords = vec![];
    let character = area[start_index.0][start_index.1];

    while to_check.len() > 0 {
        let mut tmp = vec![];

        for index in to_check {
            if area[index.0][index.1] != character {
                continue;
            }

            area[index.0][index.1] = '.';
            coords.push(index);

            if index.0 > 0 { tmp.push((index.0 - 1, index.1)); }
            if index.1 > 0 { tmp.push((index.0, index.1 -1 )); }
            if index.0 < area.len() - 1 { tmp.push((index.0 + 1, index.1)); }
            if index.1 < area[0].len() - 1 { tmp.push((index.0, index.1 + 1)); }
        }

        to_check = tmp;
    }

    Region::new(coords, area.len(), area[0].len())
}

#[derive(Debug)]
pub struct Region {
    area: HashMap<(usize, usize), Cell>
}

impl Region {
    pub fn new(coords: Vec<(usize, usize)>, width: usize, height: usize) -> Region {
        let mut cell_map: HashMap<(usize, usize), Cell> = HashMap::new();

        for coord in coords {
            let mut new_cell = Cell{ up: false, down: false, left: false, right: false };

            if coord.0 > 0 && cell_map.get(&(coord.0 - 1, coord.1)).is_some() { 
                new_cell.up = true;
                cell_map.get_mut(&(coord.0 - 1, coord.1)).unwrap().down = true;
            }

            if coord.1 > 0 && cell_map.get(&(coord.0, coord.1-1)).is_some() { 
                new_cell.left = true;
                cell_map.get_mut(&(coord.0, coord.1 -1)).unwrap().right = true;
            }

            if coord.0 < width - 1 && cell_map.get(&(coord.0 + 1, coord.1)).is_some() { 
                new_cell.down = true;
                cell_map.get_mut(&(coord.0 + 1, coord.1)).unwrap().up = true;
            }
            
            if coord.1 < height - 1 && cell_map.get(&(coord.0, coord.1+1)).is_some() { 
                new_cell.right = true;
                cell_map.get_mut(&(coord.0, coord.1+1)).unwrap().left = true;
            }

            cell_map.insert(coord, new_cell);
        }

        Region {
            area: cell_map
        }
    }

    pub fn get_cost(&self) -> u64 {
        let area = self.area.len();
        let mut fences = 0;

        for (_, cell) in self.area.iter() {
            fences += cell.get_num_fences() as u64;
        }

        area as u64 * fences
    }
}

#[derive(Debug)]
pub struct Cell {
    up: bool,
    down: bool,
    left: bool,
    right: bool
}

impl Cell {
    pub fn get_num_fences(&self) -> u8 {
        let mut fences = 0;

        if !self.up { fences += 1; }
        if !self.down { fences += 1; }
        if !self.left { fences += 1; }
        if !self.right { fences += 1; }

        fences
    }
}
