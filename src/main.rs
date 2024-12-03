use std::env;

mod day_one;
mod day_two;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        run_every_day();
        return;
    }

    let day: i64 = match args[1].parse::<i64>() {
        Ok(num) => num,
        Err(num) => {
            println!("{}", num);
            0
        },
    };

    match day {
        1 => day_one(),
        2 => day_two(),
        _ => run_every_day(),
    }
}

fn run_every_day() {
    println!("running every day...");

    day_one();
    day_two();

    println!("finished executing every day!")
}

fn day_one() {
    println!("running day one...");
    day_one::run_day_01();
}

fn day_two() {
    println!("running day one...");
    day_two::run_day_02();
}
