use std::collections::HashMap;
use crate::day_twelve::cell::Cell;

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

    pub fn get_bulk_cost(&self) -> u64 {
        let area = self.area.len();
        
        let mut l_shapes = 0;
        let mut i_shapes = 0;
        let mut inwards_corner = 0;

        for (coords, cell) in self.area.iter() {
            if cell.is_single_piece() {
                l_shapes = 4;
                break;
            }

            if cell.is_dead_end() {
                i_shapes += 1;
                continue;
            }

            if cell.is_corner() {
                l_shapes += 1;
            }

            if cell.down && cell.right && !self.area.contains_key(&(coords.0 + 1, coords.1 + 1)) {
                inwards_corner += 1;
            }

            if cell.down && cell.left && !self.area.contains_key(&(coords.0 + 1, coords.1 - 1)) {
                inwards_corner += 1;
            }

            if cell.up && cell.right && !self.area.contains_key(&(coords.0 - 1, coords.1 + 1)) {
                inwards_corner += 1;
            }

            if cell.up && cell.left && !self.area.contains_key(&(coords.0 - 1, coords.1 - 1)) {
                inwards_corner += 1;
            }
        }

        let edges = l_shapes + i_shapes * 2 + inwards_corner;

        area as u64 * edges
    }
}
