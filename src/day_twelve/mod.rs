use std::fs::read_to_string;

mod region;
mod cell;

use region::Region;

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
    let mut task_two_result = 0;
    for region in regions {
        task_one_result += region.get_cost();
        task_two_result += region.get_bulk_cost();
    }

    println!("\ttask one result: {}", task_one_result);
    println!("\ttask two result: {}", task_two_result);
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
