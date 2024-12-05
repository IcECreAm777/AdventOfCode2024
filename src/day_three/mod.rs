use regex::Regex;
use std::fs::read_to_string;

pub fn run_day_03() {
    let re_task01 = Regex::new(r"mul\((?<first>\d{1,3}),(?<second>\d{1,3})\)").unwrap();
    let re_task02 = Regex::new(r"mul\((?<first>\d{1,3}),(?<second>\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    
    let mut first_result: i64 = 0;
    let mut second_result: i64 = 0;

    let mut active = true;

    for line in read_to_string("src\\day_three\\input.txt").unwrap().lines() {
        for cap in re_task01.captures_iter(line) {
            let first = &cap["first"].parse::<i64>().unwrap();
            let second = &cap["second"].parse::<i64>().unwrap();

            first_result += first * second;
        }

        for cap in re_task02.find_iter(line) {
            match cap.as_str() {
                "do()" => {active = true;}
                "don't()" => {active = false;}
                _ => {
                    if active {
                        if let Some(mul) = re_task01.captures(cap.as_str()) {
                            let first = &mul["first"].parse::<i64>().unwrap();
                            let second = &mul["second"].parse::<i64>().unwrap();

                            second_result += first * second;
                        }
                    }
                }
            }
        }
    }

    println!("\tfirst task result: {}", first_result);
    println!("\tsecond task result: {}", second_result);
}