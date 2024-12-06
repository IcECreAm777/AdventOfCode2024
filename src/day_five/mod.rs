mod instruction_wrapper;
mod dependency;

pub fn run_day_05() {
    let path = "src\\day_five\\input_aleks.txt";

    let wrapper = instruction_wrapper::InstructionWrapper::new(path);
    let (solution_one, solution_two) = wrapper.get_line_results();

    println!("\ttask one result: {}", solution_one);
    println!("\ttask one result: {}", solution_two);
}
