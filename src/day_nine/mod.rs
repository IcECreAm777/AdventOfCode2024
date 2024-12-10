use std::{collections::HashMap, fs::read_to_string};

pub fn run_day_09() {
    let path = "src\\day_nine\\input.txt";

    let task_one_result = task01(path);
    let task_two_result = task02(path);

    println!("\ttask one result: {}", task_one_result);
    println!("\ttask two result: {}", task_two_result);
}

fn task01(path: &str) -> i64 {
    let mut disk_space: Vec<i32> = vec![];

    // parse the input
    for line in read_to_string(path).unwrap().lines() {
        let characters: Vec<char> = line.chars().collect();

        let mut index = 0;
        let mut free_space = false;

        for i in 0..characters.len() {
            let character = characters[i];
            let parsed = character.to_string().parse::<usize>().unwrap();

            if free_space {
                let mut free: Vec<i32> = vec![-1; parsed]; 
                disk_space.append(&mut free);
            } else {
                let mut occupied: Vec<i32> = vec![index; parsed];
                disk_space.append(&mut occupied);
                index += 1;
            }

            free_space = !free_space;
        }
    }

    // do the file system fragmentation

    let mut i = 0;
    let mut j = disk_space.len() - 1;

    while i < j {
        // look for next free space
        if disk_space[i] >= 0 {
            i += 1;
            continue;
        }

        // look for next entry
        if disk_space[j] < 0 {
            j -= 1;
            continue;
        }

        // swap them
        disk_space.swap(i, j);
        i += 1;
        j -= 1;
    }

    // calculate the checksum (the result)

    let mut result = 0;
    for i in 0..disk_space.len() {
        if disk_space[i] < 0 { break; }

        result += i as i64 * disk_space[i] as i64;
    }

    result
}

fn task02(path: &str) -> i64 {
    let mut disk_space: Vec<i32> = vec![];
    let mut free_spaces: Vec<(usize, usize)> = vec![];
    let mut blocks: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut index = 0;

    // parse the input
    for line in read_to_string(path).unwrap().lines() {
        let characters: Vec<char> = line.chars().collect();
        let mut free_space = false;

        for i in 0..characters.len() {
            let character = characters[i];
            let parsed = character.to_string().parse::<usize>().unwrap();

            if free_space {
                if parsed > 0 {
                    free_spaces.push((disk_space.len(), parsed));

                    let mut free: Vec<i32> = vec![-1; parsed]; 
                    disk_space.append(&mut free);
                }
            } else {
                blocks.insert(index, (disk_space.len(), parsed));

                let mut occupied: Vec<i32> = vec![index as i32; parsed];
                disk_space.append(&mut occupied);
                index += 1;
            }

            free_space = !free_space;
        }
    }

    // do the swaps

    for i in (0..index).rev() {
        let current = blocks.get(&i).unwrap();

        for j in 0..free_spaces.len() {
            let space = free_spaces[j];

            if space.0 >= current.0 { break; }
            if current.1 > space.1 { continue; }

            let new_size = space.1 - current.1;
            free_spaces[j].1 = new_size;
            free_spaces[j].0 += current.1;

            for offset in 0..current.1 {
                disk_space[current.0 + offset] = -1;
                disk_space[space.0 + offset] = i as i32;
            }

            break;
        }
    }

    // create the checksum

    let mut result = 0;

    for i in 0..disk_space.len() {
        let current = disk_space[i];
        if current < 0 { continue; }

        result += i as i64 * current as i64;
    }

    result
}
