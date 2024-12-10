use std::fs::read_to_string;

pub fn run_day_10() {
    let path = "src\\day_ten\\input.txt";
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

    println!("\ttask one result: {}", result);
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
