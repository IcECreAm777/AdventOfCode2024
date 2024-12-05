mod xmas_wrapper;

pub fn run_day_04() {
    let mut task_one = xmas_wrapper::XMasWrapper::new();

    task_one.initialize_from_file("src\\day_four\\input.txt");
    
    let result_one = task_one.find_xmas();

    println!("\ttask one result: {}", result_one);
}
