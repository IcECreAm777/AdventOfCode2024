use std::env;

mod day_one;

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
        _ => run_every_day(),
    }
}

fn run_every_day() {
    println!("running every day...");

    day_one();

    println!("finished executing every day!")
}

fn day_one() {
    println!("running day one...");
    day_one::run_day_01();
}
