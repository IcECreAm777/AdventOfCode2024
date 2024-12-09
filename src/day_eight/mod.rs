use std::{collections::HashMap, fs::read_to_string};

pub fn run_day_08() {
    let path = "src\\day_eight\\input.txt";
    let mut sorted_antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut width = 0;
    let mut height = 0;

    for line in read_to_string(path).unwrap().lines() {
        height = line.len();

        let chars:Vec<char> = line.chars().collect();

        for j in 0..chars.len() {
            let char = chars[j];
            let index = (width, j as i32);

            if char == '.' { continue; }

            let antenna_positions = sorted_antennas.get(&char);
            if antenna_positions.is_none() {
                sorted_antennas.insert(char, vec![index]); 
                continue;   
            }

            sorted_antennas.get_mut(&char).unwrap().push(index);
        }

        width += 1;
    }

    let task_one_result = task_01(sorted_antennas, width, height as i32);

    println!("\t task one result: {}", task_one_result);
}

fn task_01(sorted_antennas: HashMap<char, Vec<(i32, i32)>>, width: i32, height: i32) -> i32 {
    let mut result: Vec<(i32, i32)> = vec![];

    for (_, positions) in sorted_antennas {
        for i in 0..positions.len()-1 {
            let current = positions[i];

            for j in i+1..positions.len() {
                let next = positions[j];
                let diff = (current.0 - next.0, current.1 - next.1);

                let after_current = (current.0 + diff.0, current.1 + diff.1);
                if is_in_bound(after_current, (width, height)) && !result.contains(&after_current) {
                    result.push(after_current);
                }

                let after_next = (next.0 - diff.0, next.1 - diff.1);
                if is_in_bound(after_next, (width, height)) && !result.contains(&after_next) {
                    result.push(after_next);
                }
            }
        }
    }

    result.len() as i32
}

fn is_in_bound((x, y): (i32, i32), (width, height): (i32, i32)) -> bool {
    x >= 0 && x < width && y >= 0 && y < height
}