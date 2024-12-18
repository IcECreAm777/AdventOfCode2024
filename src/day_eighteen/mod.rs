use std::fs::read_to_string;

mod grid;

use grid::Grid;

pub fn run_day_18() {
    let path = r"src\day_eighteen\input.txt";
    let input_size = 1024;
    let mut grid = Grid::new(71);
    
    let mut i = -1;
    let mut last_path = vec![];
    for line in read_to_string(path).unwrap().lines() {
        let split: Vec<&str> = line.split(",").collect();

        let x = split[1].parse::<usize>().unwrap();
        let y = split[0].parse::<usize>().unwrap();

        i += 1;
        grid.add_corrupted_space(x, y);

        if i < input_size { continue; }

        if i == input_size {
            let grid_path = grid.a_star();
            println!("\ttask one result: {}", grid_path.len()-1);
            continue;
        }

        if last_path.contains(&(x, y)) { continue; }

        last_path = grid.a_star();

        if last_path.len() == 0 {
            println!("\ttask two result: {}", line);
            return;
        }
    }
}
