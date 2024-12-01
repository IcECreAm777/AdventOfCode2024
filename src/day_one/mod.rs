use std::fs::read_to_string;
use std::collections::HashMap;

pub fn run_day_01() {
    println!("executing task 01...");

    let mut left_list: Vec<i64> = std::vec::Vec::new();
    let mut right_list: Vec<i64>  = std::vec::Vec::new();

    for line in read_to_string("src\\day_one\\input.txt").unwrap().lines() {
        let collection: Vec<&str> = line.split(" ").collect();
        let left = collection.first().unwrap().parse::<i64>().unwrap();
        let right = collection.last().unwrap().parse::<i64>().unwrap();

        let left_pos = left_list.binary_search(&left).unwrap_or_else(|e| e);
        left_list.insert(left_pos, left);

        let right_pos = right_list.binary_search(&right).unwrap_or_else(|e| e);
        right_list.insert(right_pos, right);
    }

    let mut distance = 0;

    for i in 0..left_list.len() {
        distance += (right_list[i] - left_list[i]).abs();
    }

    println!("result for task 01: {distance}");
    println!("executing task 02...");

    let mut scores: HashMap<i64, i64> = HashMap::new();
    let mut sum_of_scores = 0;

    for i in 0..left_list.len() {
        let current_value = left_list[i];

        if scores.contains_key(&current_value) {
            sum_of_scores += scores[&current_value];
            continue;
        }

        let score = current_value * match right_list.binary_search(&current_value) {
            Ok(pos) => {
                let mut amount = 1;

                let mut index = pos - 1;
                while right_list[index] == current_value {
                    amount += 1;

                    if index == 0 {
                        break;
                    }

                    index -= 1;
                }

                index = pos + 1;
                while index < right_list.len() && right_list[index] == current_value {
                    amount += 1;
                    index += 1;
                }

                amount
            }

            Err(_) => 0
        };

        scores.insert(current_value, score);
        sum_of_scores += score;
    }

    println!("result for task 02: {}", sum_of_scores);
}