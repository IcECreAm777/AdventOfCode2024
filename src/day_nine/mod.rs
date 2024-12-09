use std::fs::read_to_string;

pub fn run_day_09() {
    let path = "src\\day_nine\\input.txt";

    for line in read_to_string(path).unwrap().lines() {
        let characters: Vec<char> = line.chars().collect();

        let mut index = 0;
        let mut disk_space: Vec<i32> = vec![];
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

        let mut result = 0;
        for i in 0..disk_space.len() {
            if disk_space[i] < 0 { break; }

            result += i as i64 * disk_space[i] as i64;
        }

        println!("\ttask one result: {}", result);
    }
}
