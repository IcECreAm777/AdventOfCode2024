mod xmas_wrapper;
mod crossed_mas_wrapper;

pub fn run_day_04() {
    let path = "src\\day_four\\input.txt";

    let task_one = xmas_wrapper::XMasWrapper::new(path);
    let result_one = task_one.find_xmas();

    println!("\ttask one result: {}", result_one);

    let task_two = crossed_mas_wrapper::CrossedMasWrapper::new(path);
    let result_two = task_two.find_crossed_mas();

    println!("\ttask two result: {}", result_two);
}
