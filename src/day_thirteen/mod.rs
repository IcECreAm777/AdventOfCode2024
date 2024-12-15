use std::fs::read_to_string;

mod crane;

use crane::Crane;

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
