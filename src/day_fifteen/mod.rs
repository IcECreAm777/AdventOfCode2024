use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
enum TileType {
    Empty,
    Wall,
    Stone
}

pub fn run_day_15() {
    let path = r"src\day_fifteen\input.txt";

    let mut ware_house: Vec<Vec<TileType>> = vec![];
    let mut instructions: Vec<char> = vec![];
    let mut robot_pos: (usize, usize) = (0, 0);

    let mut read_warehouse = true;
    let mut row: usize = 0;
    for line in read_to_string(path).unwrap().lines() {
        if line == "" {
            read_warehouse = false;
            continue;
        }

        if read_warehouse {
            let mut col: usize = 0;
            let mut new_line = vec![];
            for char in line.chars() {
                let new = match char {
                    '#' => TileType::Wall,
                    'O' => TileType::Stone,
                    '@' => {
                        robot_pos = (row, col);
                        TileType::Empty
                    },
                    _ => TileType::Empty
                };

                new_line.push(new);
                col += 1;
            }

            ware_house.push(new_line);
            row += 1;

            continue;
        }

        let mut chars: Vec<char> = line.chars().collect();
        instructions.append(&mut chars);
    }

    for instruction in instructions {
        let direction = match instruction {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => { return; }
        };

        robot_pos = move_robot(&mut ware_house, robot_pos, direction);
    }

    let mut task_one_result = 0;
    for i in 0..ware_house.len() {
        for j in 0..ware_house[i].len() {
            if ware_house[i][j] != TileType::Stone { continue; }
            task_one_result += i * 100 + j;
        }
    }

    println!("\ttask one result: {}", task_one_result);
}

fn move_robot(area: &mut Vec<Vec<TileType>>, position: (usize, usize), direction: (i8, i8)) -> (usize, usize) {
    let next_area = ((position.0 as i32 + direction.0 as i32) as usize, (position.1 as i32 + direction.1 as i32) as usize);
    
    match area[next_area.0][next_area.1] {
        TileType::Empty => next_area,
        TileType::Wall => position,
        TileType::Stone => {
            let mut i = 1;
            let mut span = ((next_area.0 as i32 + direction.0 as i32) as usize, (next_area.1 as i32 + direction.1 as i32) as usize);

            while area[span.0][span.1] == TileType::Stone {
                i += 1;
                span = ((next_area.0 as i32 + direction.0 as i32 * i) as usize, (next_area.1 as i32 + direction.1 as i32 * i) as usize);
            }

            let check_type = &area[span.0][span.1];
            if *check_type == TileType::Wall {
                return position;
            }

            area[span.0][span.1] = TileType::Stone;
            area[next_area.0][next_area.1] = TileType::Empty;

            return next_area;
        }
    }
}