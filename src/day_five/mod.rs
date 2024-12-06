mod instruction_wrapper;
mod dependency;

pub fn run_day_05() {
    let path = "src\\day_five\\input.txt";

    let wrapper = instruction_wrapper::InstructionWrapper::new(path);
    let solution_one = wrapper.get_valid_lines();

    println!("\ttask one result: {}", solution_one);
}
