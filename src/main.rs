use std::env;
use std::time::Instant;

mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod day_five;
mod day_six;
mod day_seven;

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
        6 => day_six(),
        7 => day_seven(),
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
    day_six();
    day_seven();

    println!("finished executing every day!")
}

fn day_one() {
    println!("running day one...");
    let start = Instant::now();
    day_one::run_day_01();
    println!("executing day took {} ms", start.elapsed().as_millis());
}

fn day_two() {
    println!("running day two...");
    let start = Instant::now();
    day_two::run_day_02();
    println!("executing day took {} ms", start.elapsed().as_millis());
}

fn day_three() {
    println!("running day three...");
    let start = Instant::now();
    day_three::run_day_03();
    println!("executing day took {} ms", start.elapsed().as_millis());
}

fn day_four() {
    println!("running day four...");
    let start = Instant::now();
    day_four::run_day_04();
    println!("executing day took {} ms", start.elapsed().as_millis());
}

fn day_five() {
    println!("running day five...");
    let start = Instant::now();
    day_five::run_day_05();
    println!("executing day took {} ms", start.elapsed().as_millis());
}

fn day_six() {
    println!("running day six...");
    let start = Instant::now();
    day_six::run_day_06();
    println!("executing day took {} ms", start.elapsed().as_millis());
}

fn day_seven() {
    println!("running day seven...");
    let start = Instant::now();
    day_seven::run_day_07();
    println!("executing day took {} ms", start.elapsed().as_millis());
}
