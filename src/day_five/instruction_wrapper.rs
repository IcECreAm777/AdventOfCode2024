use std::{collections::HashMap, fs::read_to_string};
use crate::day_five::dependency::Dependency;

pub struct InstructionWrapper {
    pages: Vec<Vec<i32>>,
    dependencies: HashMap<i32, Dependency>
}

impl InstructionWrapper {
    pub fn new(path: &str) -> InstructionWrapper {
        let mut read_dependency = true;
        let mut pages = vec![];
        let mut dependencies: HashMap<i32, Dependency> = HashMap::new();

        for line in read_to_string(path).unwrap().lines() {
            if line == "" {
                read_dependency = false;
                continue;
            }

            if read_dependency {
                let split: Vec<&str> = line.split("|").collect();
                let first = split[0].parse::<i32>().unwrap();
                let second = split[1].parse::<i32>().unwrap();

                if !dependencies.contains_key(&first) {
                    let dep = Dependency::new();
                    dependencies.insert(first, dep);
                }

                dependencies.get_mut(&first).unwrap().add_next(second);

                continue;
            }

            let mut new_vec = vec![];
            for element in line.split(",") {
                let num = element.parse::<i32>().unwrap();
                new_vec.push(num);
            }

            pages.push(new_vec);
        }

        return InstructionWrapper {
            pages,
            dependencies
        }
    }

    pub fn get_line_results(self) -> (i64, i64) {
        let mut valid_result: i64 = 0;
        let mut invalid_result: i64 = 0;

        for mut line in self.pages {
            let mut is_valid = true;

            for i in 1..line.len() {
                for j in 0..i {
                    let prev = line[j];
                    let current = line[i];
                    let dep = self.dependencies.get(&current).unwrap();
                    
                    if dep.is_next(prev) {
                        line.swap(i, j);
                        is_valid = false;
                    }
                }
            }

            let index = line.len() / 2;
            let middle_page = line[index];

            if is_valid {               
                valid_result += middle_page as i64;
            } else {
                invalid_result += middle_page as i64;
                println!("{:#?}", line);
            }
        }

        (valid_result, invalid_result)
    }
}
