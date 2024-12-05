mod xmas_wrapper;
mod crossed_mas_wrapper;

pub fn run_day_04() {
    let task_one = xmas_wrapper::XMasWrapper::new("src\\day_four\\input.txt");
    let result_one = task_one.find_xmas();

    println!("\ttask one result: {}", result_one);


}
