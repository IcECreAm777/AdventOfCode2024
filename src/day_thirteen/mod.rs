use std::fs::read_to_string;

pub fn run_day_13() {
    let path = r"src\day_thirteen\input.txt";

    let mut line_num = 0;
    let mut cranes = vec![];
    let mut current_crane = Crane::new();
    let button_regex = regex::Regex::new(r"\+(?<x>\d*), Y\+(?<y>\d*)").unwrap();
    let target_regex = regex::Regex::new(r"=(?<x>\d*), Y=(?<y>\d*)").unwrap();

    for line in read_to_string(path).unwrap().lines() {
        match line_num % 4 {
            0 => {
                let capture = button_regex.captures(line).unwrap();
                let x = capture["x"].parse().unwrap();
                let y = capture["y"].parse().unwrap();
                current_crane.set_a_button(x, y);
            },
            1 => {
                let capture = button_regex.captures(line).unwrap();
                let x = capture["x"].parse().unwrap();
                let y = capture["y"].parse().unwrap();
                current_crane.set_b_button(x, y);
            },
            2 => {
                let capture = target_regex.captures(line).unwrap();
                let x = capture["x"].parse().unwrap();
                let y = capture["y"].parse().unwrap();
                current_crane.set_result(x, y);
            },
            3 => {
                cranes.push(current_crane);
                current_crane = Crane::new()
            },
            _ => {}
        }

        line_num += 1;
    }

    cranes.push(current_crane);

    let mut task_one_result = 0;
    let mut task_two_result = 0;
    for mut crane in cranes {
        task_one_result += crane.solve();

        crane.correct_result();
        task_two_result += crane.solve();
    }

    println!("\ttask one result: {}", task_one_result);
    println!("\ttask one result: {}", task_two_result);
}

#[derive(Debug)]
pub struct Crane {
    a_x: u64,
    a_y: u64,
    b_x: u64,
    b_y: u64,
    r_x: u64,
    r_y: u64
}

impl Crane {
    pub fn new() -> Crane {
        Crane { a_x: 0, a_y: 0, b_x: 0, b_y: 0, r_x: 0, r_y: 0 }
    }

    pub fn set_a_button(&mut self, x: u64, y: u64) {
        self.a_x = x;
        self.a_y = y;
    }

    pub fn set_b_button(&mut self, x: u64, y: u64) {
        self.b_x = x;
        self.b_y = y;
    }

    pub fn set_result(&mut self, x:u64, y: u64) {
        self.r_x = x;
        self.r_y = y;
    }

    pub fn correct_result(&mut self) {
        self.r_x += 10000000000000;
        self.r_y += 10000000000000;
    }

    pub fn solve(&self) -> u64 {
        let det_a = (self.a_x * self.b_y) as i128 - (self.b_x * self.a_y) as i128;
        let det_one = (self.r_x * self.b_y) as i128 - (self.b_x * self.r_y) as i128;
        let det_two = (self.a_x * self.r_y) as i128 - (self.r_x * self.a_y) as i128;

        if det_one % det_a != 0 || det_two % det_a != 0 {
            return 0
        }

        let solution_one = det_one / det_a;
        let solution_two = det_two / det_a;

        if solution_one < 0 || solution_two < 0 {
            return 0;
        }

        solution_one as u64 * 3 + solution_two as u64
    }
}
