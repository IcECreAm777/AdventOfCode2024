mod area;

pub fn run_day_06() {
    let path = "src\\day_six\\input.txt";
    
    let mut task_one_area = area::Area::new(&path);
    let task_one_result = task_one_area.get_covered_area();

    println!("\ttask one result: {}", task_one_result);
}
