use std::fs::read_to_string;

pub fn run_day_10() {
    let path = "src\\day_ten\\input.txt";
    
    let (area, zero_index) = parse_input(path);
    let task_one_result = task01(&area, zero_index.clone());
    let task_two_result = task02(&area, zero_index.clone());

    println!("\ttask one result: {}", task_one_result);
    println!("\ttask two result: {}", task_two_result);
}

fn parse_input(path: &str) -> (Vec<Vec<i8>>, Vec<(usize, usize)>) {
    let mut area: Vec<Vec<i8>> = vec![];
    let mut zero_index: Vec<(usize, usize)> = vec![];

    let mut row_index = 0;
    for line in read_to_string(path).unwrap().lines() {
        let mut row = vec![];
        let mut col_index = 0;

        for c in line.chars() {
            let num = c.to_string().parse::<i8>().unwrap();
            row.push(num);

            if num == 0 {
                zero_index.push((row_index, col_index));
            }

            col_index += 1;
        }

        area.push(row);

        row_index += 1;
    }

    (area, zero_index)
}

fn task01(area: &Vec<Vec<i8>>, zero_index: Vec<(usize, usize)>) -> i32 {
    let mut result = 0;
    for zero in zero_index {
        let mut index = 0;
        let mut next = vec![zero];

        while next.len() > 0 && index < 9 {
            let mut tmp = vec![];

            for n in next {
                for g in get_next_index(&area, n, index) {
                    if !tmp.contains(&g) {tmp.push(g);}
                }
            }

            next = tmp;
            index += 1;
        }

        result += next.len() as i32;
    } 

    result
}

fn task02(area: &Vec<Vec<i8>>, zero_index: Vec<(usize, usize)>) -> i32 {
    let mut result = 0;

    for zero in zero_index {
        result += traverse_to_next(area, zero, 0);
    }

    result
}

fn get_next_index(area: &Vec<Vec<i8>>, coords: (usize, usize), index: i8) -> Vec<(usize, usize)> {
    let mut next: Vec<(usize, usize)> = vec![];

    if coords.0 > 0 && area[coords.0 - 1][coords.1] == index + 1 {
        next.push((coords.0-1, coords.1));
    } 

    if coords.1 > 0 && area[coords.0][coords.1-1] == index + 1 {
        next.push((coords.0, coords.1-1));
    }

    if coords.0 < area.len()-1 && area[coords.0 + 1][coords.1] == index + 1 {
        next.push((coords.0 + 1, coords.1));
    }

    if coords.1 < area[0].len()-1 && area[coords.0][coords.1+1] == index + 1 {
        next.push((coords.0, coords.1+1));
    }

    next
}

fn traverse_to_next(area: &Vec<Vec<i8>>, coords: (usize, usize), index: i8) -> i32 {
    let current = area[coords.0][coords.1];
    if current != index {
        return 0
    }

    if index == 9 {
        return 1;
    }

    let mut result = 0;

    if coords.0 > 0 && area[coords.0 - 1][coords.1] == index + 1 {
        result += traverse_to_next(area, (coords.0 - 1, coords.1), index + 1);
    } 

    if coords.1 > 0 && area[coords.0][coords.1-1] == index + 1 {
        result += traverse_to_next(area, (coords.0, coords.1-1), index + 1);
    }

    if coords.0 < area.len()-1 && area[coords.0 + 1][coords.1] == index + 1 {
        result += traverse_to_next(area, (coords.0 + 1, coords.1), index + 1);
    }

    if coords.1 < area[0].len()-1 && area[coords.0][coords.1+1] == index + 1 {
        result += traverse_to_next(area, (coords.0, coords.1 + 1), index + 1);
    }

    result
}
