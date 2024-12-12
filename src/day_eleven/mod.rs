use std::fs::read_to_string;
use memoize::memoize;

pub fn run_day_11() {
    let path = "src\\day_eleven\\input.txt";
    
    let mut stones: Vec<u64> = vec![];
    for num in read_to_string(path).unwrap().split_whitespace() {
        let parsed = num.parse::<u64>().unwrap();
        stones.push(parsed);
    }

    let mut task_one_result = stones.len() as u128;
    let mut task_two_result = stones.len() as u128;

    for stone in stones {
        task_one_result += solve(stone, 0, 25);
        task_two_result += solve(stone, 0, 75);
    }

    println!("\ttask one result: {}", task_one_result);
    println!("\ttask two result: {}", task_two_result);
}

#[memoize]
fn solve(number: u64, iteration: u16, max_depth: u16) -> u128 {
    if iteration >= max_depth {
        return 0;
    }

    if number == 0 {
        return solve(1, iteration+1, max_depth);
    }

    let as_string = number.to_string();
    if as_string.len() % 2 == 0 {
        let mut total = 1;

        let (first, second) = as_string.split_at(as_string.len() / 2);
        
        total += solve(first.parse().unwrap(), iteration+1, max_depth);
        total += solve(second.parse().unwrap(), iteration+1, max_depth);

        return total;
    }

    solve(number*2024, iteration+1, max_depth)
}
