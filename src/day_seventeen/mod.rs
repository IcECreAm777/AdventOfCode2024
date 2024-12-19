mod computer;
mod reverse_solver;

use computer::Computer;
use reverse_solver::ReverseSolver;

pub fn run_day_17() {
    let mut computer = Computer::new(63687530, 0, 0, vec![2,4,1,3,7,5,0,3,1,5,4,1,5,5,3,0]);
    let task_one_result = computer.solve_program();

    let mut reverser = ReverseSolver::new(vec![0,3,5,4,3,0]);
    let task_two_result = reverser.solve();

    println!("\ttask one result: {}", task_one_result);
    println!("\ttask two result: {}", task_two_result);
}
