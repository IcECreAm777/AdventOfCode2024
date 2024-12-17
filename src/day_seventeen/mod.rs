pub fn run_day_17() {
    let mut computer = Computer::new(63687530, 0, 0, vec![2,4,1,3,7,5,0,3,1,5,4,1,5,5,3,0]);
    let task_one_result = computer.solve_program();

    println!("\ttask one result: {}", task_one_result);
}

pub struct Computer {
    a: u64,
    b: u64,
    c: u64,
    instructions: Vec<u8>,
}

impl Computer {
    pub fn new(a: u64, b: u64, c:u64, instructions: Vec<u8>) -> Computer {
        Computer { a, b, c, instructions }
    }

    pub fn solve_program(&mut self) -> String {
        let mut result = vec![];
        let mut instruction_pointer = 0;

        while instruction_pointer < self.instructions.len()-1  {
            let operation = self.instructions[instruction_pointer];
            let value = self.instructions[instruction_pointer+1];
    
            match operation {
                0 => {
                    let combo_value = self.get_combo_value(value);
                    let base: u64 = 2;
                    let denominator = base.pow(combo_value as u32);

                    self.a = self.a / denominator;
                },
                1 => {
                    self.b = self.b ^ value as u64;
                },
                2 => {
                    let combo_value = self.get_combo_value(value);
                    self.b = combo_value % 8;
                },
                3 => {
                    if self.a != 0  {
                        instruction_pointer = value as usize;
                        continue;
                    }
                },
                4 => {
                    self.b = self.b ^ self.c;
                },
                5 => {
                    let combo_value = self.get_combo_value(value);
                    result.push((combo_value % 8) as u8);
                }
                6 => {
                    let combo_value = self.get_combo_value(value);
                    let base: u64 = 2;
                    let denominator = base.pow(combo_value as u32);

                    self.b = self.a / denominator;
                },
                7 => {
                    let combo_value = self.get_combo_value(value);
                    let base: u64 = 2;
                    let denominator = base.pow(combo_value as u32);

                    self.c = self.a / denominator;
                },
                _ => { return String::new(); }
            }

            instruction_pointer += 2;
        }

        let mut result_string = result[0].to_string();
        for i in 1..result.len() {
            let new_value = result[i];
            result_string = format!("{},{}", result_string, new_value.to_string());
        }

        result_string
    }

    fn get_combo_value(&self, literal: u8) -> u64 {
        match literal {
            0 | 1 | 2 | 3 => literal as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => 0
        }
    }
}