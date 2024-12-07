mod simple_area;
mod complex_area;

pub fn run_day_06() {
    let path = "src\\day_six\\input_aleks.txt";
    
    let mut task_one_area = simple_area::SimpleArea::new(&path);
    let task_one_result = task_one_area.get_covered_area();

    let mut task_two_area = complex_area::ComplexArea::new(&path);
    let task_two_result = task_two_area.get_num_possible_loops();

    println!("\ttask one result: {}", task_one_result);
    println!("\ttask two result: {}", task_two_result);
}
