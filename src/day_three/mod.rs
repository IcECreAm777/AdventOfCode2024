use regex::Regex;
use std::fs::read_to_string;

pub fn run_day_03() {
    let re_task01 = Regex::new(r"mul\((?<first>\d{1,3}),(?<second>\d{1,3})\)").unwrap();
    let re_task02 = Regex::new(r"mul\((?<first>\d{1,3}),(?<second>\d{1,3})\)|(?<do>do\(\))|(?<dont>don't\(\))").unwrap();
    
    let mut first_result: i64 = 0;
    let mut second_result: i64 = 0;

    for line in read_to_string("src\\day_three\\input.txt").unwrap().lines() {
        for cap in re_task01.captures_iter(line) {
            let first = &cap["first"].parse::<i64>().unwrap();
            let second = &cap["second"].parse::<i64>().unwrap();

            first_result += first * second;
        }

        let mut active = true;
        for cap in re_task02.captures_iter(line) {
            match cap.name("do") {
                Some(_) => {
                    active = true;
                    continue;
                },
                None => ()
            }

            match cap.name("dont") {
                Some(_) => {
                    active = false;
                    continue;
                },
                None => ()
            }

            if active {
                let first = &cap["first"].parse::<i64>().unwrap();
                let second = &cap["second"].parse::<i64>().unwrap();

                second_result += first * second;
            }
        }
    }

    println!("\tfirst task result: {}", first_result);
    println!("\tsecond task result: {}", second_result);
}