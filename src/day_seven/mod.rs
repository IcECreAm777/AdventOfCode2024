use std::fs::read_to_string;

pub fn run_day_07() {
    let path = r"src\day_seven\input.txt";
    
    let task_one_result = task_01(path);
    let task_two_result = task_02(path);

    println!("\ttask one result: {}", task_one_result);
    println!("\ttask two result: {}", task_two_result);
}

fn task_01(path: &str) -> i64 {
    let mut result = 0;

    for line in read_to_string(path).unwrap().lines() {
        let split_by_colon: Vec<&str> = line.split(":").collect();
        let target_result = split_by_colon[0].parse::<i64>().unwrap();
        let numbers: Vec<i64> = split_by_colon[1].split_whitespace().map(|num| num.parse::<i64>().unwrap()).collect();

        for i in 0..1 << numbers.len() {
            let mut intermediate_result: i128 = numbers[0] as i128;
    
            for j in 1..numbers.len() {
                let target_flag = 1 << j - 1;
                let flag = target_flag & i > 0;
    
                intermediate_result = if flag {intermediate_result + numbers[j] as i128} else {intermediate_result * numbers[j] as i128}; 
            }
    
            if intermediate_result == target_result as i128 {
                result += target_result;
                break;
            }
        }
    }

    result
}

fn task_02(path: &str) -> i64 {
    let mut result = 0;

    for line in read_to_string(path).unwrap().lines() {
        let split_by_colon: Vec<&str> = line.split(":").collect();
        let target_result = split_by_colon[0].parse::<i64>().unwrap();
        let numbers: Vec<&str> = split_by_colon[1].split_whitespace().collect();
        let operators = Operators::new(numbers.len());

        for op in operators {
            let mut intermediate_result = numbers[0].parse::<i128>().unwrap();
            for i in 1..numbers.len() {
                match op[i-1] {
                    0 => { intermediate_result += numbers[i].parse::<i128>().unwrap() }
                    1 => { intermediate_result *= numbers[i].parse::<i128>().unwrap() }
                    2 => { intermediate_result = (intermediate_result.to_string() + numbers[i]).parse::<i128>().unwrap() }
                    _ => { println!("invalid operation")}
                }
            }

            // println!("{:#?} -> {} =?= {}", op, target_result, intermediate_result);
            if intermediate_result == target_result as i128 {
                result += target_result;
                break;
            }
        }
    }

    result
}

struct Operators {
    operators: Vec<i8>
}

impl Operators {
    fn new(len: usize) -> Operators {
        let mut ops = vec![0; len-1];
        ops[0] = -1;

        Operators { 
            operators: ops
        }
    }
}

impl Iterator for Operators {
    type Item = Vec<i8>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut over = 0;

        for i in 0..self.operators.len() {
            over = 0;
            self.operators[i] += 1;

            if self.operators[i] < 3 {
                break;
            }

            self.operators[i] = 0;
            over = 1;
        }

        if over == 0 {Some(self.operators.clone())} else {None}
    }
}
