use std::fs::read_to_string;

pub fn run_day_02() {
    let mut num_safe = 0;
    let mut error_safe = 0;

    for line in read_to_string("src\\day_two\\input.txt").unwrap().lines() {
        if check_is_safe(line) {
            num_safe += 1;
        }

        if check_safe_with_error(line) {
            error_safe += 1;
        }
    }

    println!("\ttask 01 result: {}", num_safe);
    println!("\ttask 02 result: {}", error_safe);
}

fn check_is_safe(line: &str) -> bool {
    let mut entries = std::vec::Vec::new();
    let strings: Vec<&str> = line.split(" ").collect();

    for entry in strings {
        let parsed = entry.parse::<i64>().unwrap();
        entries.push(parsed);
    }

    check_list(entries)
}

fn check_safe_with_error(line: &str) -> bool {
    let mut entries = std::vec::Vec::new();
    let strings: Vec<&str> = line.split(" ").collect();

    for entry in strings {
        let parsed = entry.parse::<i64>().unwrap();
        entries.push(parsed);
    }

    if check_list(entries.clone()) {
        return true;
    }

    for i in 0..entries.len() {
        let mut current = entries.clone();
        current.remove(i);

        if check_list(current) {
            return true;
        }
    }

    false
}

fn check_list(entries: std::vec::Vec<i64>) -> bool {
    let diff = entries[1] - entries[0];

    let lower_bound = if diff < 0 { -3 } else { 1 };
    let upper_bound = if diff < 0 { -1 } else { 3 };

    for i in 0..entries.len()-1 {
        let diff = entries[i+1] - entries[i];

        if diff < lower_bound || diff > upper_bound {
            return false;
        }
    }

    true
}
