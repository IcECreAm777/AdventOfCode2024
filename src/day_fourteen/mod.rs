use std::fs::read_to_string;

mod robot;

use robot::Robot;

pub fn run_day_14() {
    let path = r"src\day_fourteen\input.txt";

    let mut robots: Vec<Robot> = vec![];

    for line in read_to_string(path).unwrap().lines() {
        let split: Vec<&str> = line.split_whitespace().collect();

        let p = split[0].replace("p=", "");
        let split_p: Vec<&str> = p.split(",").collect();
        let coords = (split_p[0].parse::<usize>().unwrap(), split_p[1].parse::<usize>().unwrap());

        let v = split[1].replace("v=", "");
        let split_v: Vec<&str> = v.split(",").collect();
        let velocity = (split_v[0].parse::<i32>().unwrap(), split_v[1].parse::<i32>().unwrap());

        robots.push(Robot::new(coords, velocity));
    }

    let width = 101;
    let height = 103;
    
    let task_one_result = solve_task_01(width, height, &robots);
    let task_two_result = solve_task_02(width, height, &robots);

    println!("\ttask one result: {}", task_one_result);
    println!("\ttask two result: {}", task_two_result);
}

fn solve_task_01(width: i32, height: i32, robots: &Vec<Robot>) -> u64 {
    let mut quadrants = vec![0; 4];

    for robot in robots {
        let coords = robot.move_robot(width, height, 100);
        let index = get_quadrant_index(coords.0, coords.1, width, height);
        if index < 0 { continue; }

        quadrants[index as usize] += 1;
    }

    quadrants.into_iter().product()
}

fn solve_task_02(width: i32, height: i32, robots: &Vec<Robot>) -> usize {
    for i in 1..11000 {
        let mut all_coords = vec![];
        let mut can_be_processed = true;

        for robot in robots.iter() {
            let coords = robot.move_robot(width, height, i);
            if all_coords.contains(&coords) {
                can_be_processed = false;
                break;
            }

            all_coords.push(coords);
        }

        if can_be_processed { 
            return i;
        }
    }

    0
}

fn get_quadrant_index(x: i32, y: i32, width: i32, height: i32) -> i8 {
    if x < width / 2  {
        if y < height / 2 {
            return 0;
        } else if y > height / 2  {
            return 1;
        }
    } else if x > width / 2 {
        if y < height / 2 {
            return 2;
        } else if y > height / 2 {
            return 3
        }
    }

    -1
}
