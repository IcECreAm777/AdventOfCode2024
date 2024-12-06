use std::env;

mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod day_five;

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
        3 => day_three(),
        4 => day_four(),
        5 => day_five(),
        _ => run_every_day(),
    }
}

fn run_every_day() {
    println!("running every day...");

    day_one();
    day_two();
    day_three();
    day_four();
    day_five();

    println!("finished executing every day!")
}

fn day_one() {
    println!("running day one...");
    day_one::run_day_01();
}

fn day_two() {
    println!("running day two...");
    day_two::run_day_02();
}

fn day_three() {
    println!("running day three...");
    day_three::run_day_03();
}

fn day_four() {
    println!("running day four...");
    day_four::run_day_04();
}

fn day_five() {
    println!("running day five...");
    day_five::run_day_05();
}
