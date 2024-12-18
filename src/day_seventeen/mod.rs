mod computer;

use computer::Computer;

pub fn run_day_17() {
    let mut computer = Computer::new(63687530, 0, 0, vec![2,4,1,3,7,5,0,3,1,5,4,1,5,5,3,0]);
    let task_one_result = computer.solve_program();

    println!("\ttask one result: {}", task_one_result);
}
