use std::fs::read_to_string;

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
    let mut quadrants = vec![0; 4];

    for robot in robots {
        let coords = robot.move_robot(width, height);

        if coords.0 < width / 2  {
            if coords.1 < height / 2 {
                quadrants[0] += 1;
            } else if coords.1 > height / 2  {
                quadrants[1] += 1;
            }
        } else if coords.0 > width / 2 {
            if coords.1 < height / 2 {
                quadrants[2] += 1;
            } else if coords.1 > height / 2 {
                quadrants[3] += 1;
            }
        }
    }

    let result: u64 = quadrants.into_iter().product();
    println!("\ttask one result: {}", result);
}

#[derive(Debug)]
pub struct Robot {
    start_point: (usize, usize),
    velocity: (i32, i32)
}

impl Robot {
    pub fn new(start: (usize, usize), vel: (i32, i32)) -> Robot {
        Robot {
            start_point: start,
            velocity: vel
        }
    }

    pub fn move_robot(&self, width: i32, height: i32) -> (i32, i32) {
        let new_point = (self.start_point.0 as i32 + self.velocity.0 * 100, self.start_point.1 as i32 + self.velocity.1 * 100);

        let raw_x = new_point.0 % width;
        let raw_y = new_point.1 % height;

        let x = if raw_x < 0 { width - raw_x.abs() } else { raw_x };
        let y = if raw_y < 0 { height - raw_y.abs() } else { raw_y };

        (x, y)
    }
}
