use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
enum TileState {
    Empty,
    Obstacle,
    Visited
}

pub fn run_day_06() {

    let mut guard_position = (0, 0);
    let mut area = vec![];

    let mut i: usize = 0;
    for line in read_to_string("src\\day_six\\input.txt").unwrap().lines() {
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

    let border = (area.len(), area[0].len());
    let mut walk_direction = (-1, 0);
    let mut counter = 1;

    while !is_leaving_area(guard_position, walk_direction, border) {
        let next_area_index = ((guard_position.0 as i32 + walk_direction.0) as usize, (guard_position.1 as i32 + walk_direction.1) as usize);
        let next_tile = &area[next_area_index.0][next_area_index.1];

        if next_tile == &TileState::Obstacle {
            walk_direction = turn_direction(walk_direction);
            continue;
        }

        if next_tile == &TileState::Empty {
            counter += 1;
        }

        guard_position = next_area_index; 
        area[next_area_index.0][next_area_index.1] = TileState::Visited;
    }

    println!("\ttask one result: {}", counter);
}

fn turn_direction((x, y): (i32, i32)) -> (i32, i32) {
    let new_x = y;
    let new_y = x * -1;

    (new_x, new_y)
}

fn is_leaving_area(guard_pos: (usize, usize), direction: (i32, i32), border: (usize, usize)) -> bool {
    return guard_pos.0 == 0 && direction.0 == -1 || guard_pos.1 == 0 && direction.1 == -1
        || guard_pos.0 == border.0-1 && direction.0 == 1 || guard_pos.1 == border.1-1 && direction.1 == 1;
}
