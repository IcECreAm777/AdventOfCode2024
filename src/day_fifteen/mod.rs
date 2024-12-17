use std::fs::read_to_string;

#[derive(Debug, PartialEq, Clone, Copy)]
enum TileType {
    Empty,
    Wall,
    Stone,
    StoneLeft,
    StoneRight,
}

pub fn run_day_15() {
    let path = r"src\day_fifteen\input.txt";

    let mut ware_house_one: Vec<Vec<TileType>> = vec![];
    let mut ware_house_two: Vec<Vec<TileType>> = vec![];
    let mut instructions: Vec<char> = vec![];
    let mut first_robot_pos: (usize, usize) = (0, 0);
    let mut second_robot_pos: (usize, usize) = (0, 0);

    let mut read_warehouse = true;
    let mut row: usize = 0;
    for line in read_to_string(path).unwrap().lines() {
        if line == "" {
            read_warehouse = false;
            continue;
        }

        if read_warehouse {
            let mut col: usize = 0;
            let mut new_first = vec![];
            let mut new_second = vec![];

            for char in line.chars() {
                let (one, two) = match char {
                    '#' => (TileType::Wall, (TileType::Wall, TileType::Wall)),
                    'O' => (TileType::Stone, (TileType::StoneLeft, TileType::StoneRight)),
                    '@' => {
                        first_robot_pos = (row, col);
                        second_robot_pos = (row, col*2);
                        (TileType::Empty, (TileType::Empty, TileType::Empty))
                    },
                    _ => (TileType::Empty, (TileType::Empty, TileType::Empty))
                };

                new_first.push(one);
                new_second.push(two.0);
                new_second.push(two.1);

                col += 1;
            }

            ware_house_one.push(new_first);
            ware_house_two.push(new_second);

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

        first_robot_pos = move_robot(&mut ware_house_one, first_robot_pos, direction);
        second_robot_pos = move_robot(&mut ware_house_two, second_robot_pos, direction);
    }

    let mut task_one_result = 0;
    for i in 0..ware_house_one.len() {
        for j in 0..ware_house_one[i].len() {
            if ware_house_one[i][j] != TileType::Stone { continue; }
            task_one_result += i * 100 + j;
        }
    }

    let mut task_two_result = 0;
    for i in 1..ware_house_two.len() {
        for j in 1..ware_house_two[i].len() {
            if ware_house_two[i][j] != TileType::StoneLeft { continue; }
            task_two_result += i * 100 + j;
        }
    }

    println!("\ttask one result: {}", task_one_result);
    println!("\ttask two result: {}", task_two_result);
}

fn move_robot(area: &mut Vec<Vec<TileType>>, position: (usize, usize), direction: (i8, i8)) -> (usize, usize) {    
    let next_area = ((position.0 as i32 + direction.0 as i32) as usize, (position.1 as i32 + direction.1 as i32) as usize);

    match area[next_area.0][next_area.1] {
        TileType::Empty => next_area,
        TileType::Wall => position,
        TileType::Stone => move_single_stone(area, position, direction),
        TileType::StoneRight | TileType::StoneLeft => {
            if direction.0 == 0 {
                return move_double_stones_horizontally(area, position, direction);
            }

            move_double_stones_vertically(area, position, direction)
        }
    }
}

fn move_single_stone(area: &mut Vec<Vec<TileType>>, position: (usize, usize), direction: (i8, i8)) -> (usize, usize) {
    let next_area = ((position.0 as i32 + direction.0 as i32) as usize, (position.1 as i32 + direction.1 as i32) as usize);
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

fn move_double_stones_horizontally(area: &mut Vec<Vec<TileType>>, position: (usize, usize), direction: (i8, i8)) -> (usize, usize) {
    let next_area = ((position.0 as i32 + direction.0 as i32) as usize, (position.1 as i32 + direction.1 as i32) as usize);

    let mut i = 1;
    let mut span = ((next_area.0 as i32 + direction.0 as i32) as usize, (next_area.1 as i32 + direction.1 as i32) as usize);

    while area[span.0][span.1] == TileType::StoneLeft || area[span.0][span.1] == TileType::StoneRight {
        i += 1;
        span = ((next_area.0 as i32 + direction.0 as i32 * i) as usize, (next_area.1 as i32 + direction.1 as i32 * i) as usize);
    }

    let check_type = &area[span.0][span.1];
    if *check_type == TileType::Wall {
        return position;
    }

    while span.1 != next_area.1 {
        let next_col = (span.1 as i32 - direction.1 as i32) as usize;
        area[span.0][span.1] = area[span.0][next_col];
        span.1 = next_col;
    }

    area[next_area.0][next_area.1] = TileType::Empty;

    return next_area;
}

fn move_double_stones_vertically(area: &mut Vec<Vec<TileType>>, position: (usize, usize), direction: (i8, i8)) -> (usize, usize) {
    let next_area = ((position.0 as i32 + direction.0 as i32) as usize, (position.1 as i32 + direction.1 as i32) as usize);

    let (left, right) = if area[next_area.0][next_area.1] == TileType::StoneLeft {
        (next_area, (next_area.0, next_area.1 + 1))
    } else {
        ((next_area.0, next_area.1 - 1), next_area)
    };

    if can_be_pushed_vertically(area, left, right, direction) {
        push_stones_vertically(area, left, right, direction);
        return next_area;
    }

    position
}

fn can_be_pushed_vertically(area: &mut Vec<Vec<TileType>>, left_position: (usize, usize), right_position: (usize, usize), direction: (i8, i8)) -> bool {
    let next_left = ((left_position.0 as i32 + direction.0 as i32) as usize, (left_position.1 as i32 + direction.1 as i32) as usize);
    let next_right = ((right_position.0 as i32 + direction.0 as i32) as usize, (right_position.1 as i32 + direction.1 as i32) as usize);
    
    if area[next_left.0][next_left.1] == TileType::Empty && area[next_right.0][next_right.1] == TileType::Empty {
        return true;
    }

    if area[next_left.0][next_left.1] == TileType::Wall || area[next_right.0][next_right.1] == TileType::Wall {
        return false;
    }

    if area[next_left.0][next_left.1] == TileType::StoneLeft && area[next_right.0][next_right.1] == TileType::StoneRight {
        can_be_pushed_vertically(area, next_left, next_right, direction)
    } else {
        let next_actual_left = (next_left.0, next_left.1 - 1);
        let next_actual_right = (next_right.0, next_right.1 + 1);

        let left_ok = match area[next_left.0][next_left.1]  {
            TileType::Empty => true,
            TileType::Wall => false,
            TileType::StoneRight => can_be_pushed_vertically(area, next_actual_left, next_left, direction),
            _ => false
        };

        let right_ok = match area[next_right.0][next_right.1] {
            TileType::Empty => true,
            TileType::Wall => false,
            TileType::StoneLeft => can_be_pushed_vertically(area, next_right, next_actual_right, direction),
            _ => false
        };

        left_ok && right_ok
    }
}

fn push_stones_vertically(area: &mut Vec<Vec<TileType>>, left_position: (usize, usize), right_position: (usize, usize), direction: (i8, i8)) {
    let next_left = ((left_position.0 as i32 + direction.0 as i32) as usize, (left_position.1 as i32 + direction.1 as i32) as usize);
    let next_right = ((right_position.0 as i32 + direction.0 as i32) as usize, (right_position.1 as i32 + direction.1 as i32) as usize);

    if area[next_left.0][next_left.1] != TileType::Empty || area[next_right.0][next_right.1] != TileType::Empty {
        if area[next_left.0][next_left.1] == TileType::StoneLeft && area[next_right.0][next_right.1] == TileType::StoneRight {
            push_stones_vertically(area, next_left, next_right, direction);
        } else {
            let next_actual_left = (next_left.0, next_left.1 - 1);
            let next_actual_right = (next_right.0, next_right.1 + 1);

            if area[next_left.0][next_left.1] != TileType::Empty && area[next_left.0][next_left.1] != TileType::Wall {
                push_stones_vertically(area, next_actual_left, next_left, direction);
            }

            if area[next_right.0][next_right.1] != TileType::Empty && area[next_right.0][next_right.1] != TileType::Wall {
                push_stones_vertically(area, next_right, next_actual_right, direction);
            }
        }
    }

    area[next_left.0][next_left.1] = area[left_position.0][left_position.1];
    area[next_right.0][next_right.1] = area[right_position.0][right_position.1];

    area[left_position.0][left_position.1] = TileType::Empty;
    area[right_position.0][right_position.1] = TileType::Empty;
}