use std::env;
use std::time::Instant;

mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod day_five;
mod day_six;
mod day_seven;
mod day_eight;
mod day_nine;
mod day_ten;
mod day_eleven;
mod day_twelve;
mod day_thirteen;
mod day_fourteen;
mod day_fifteen;
mod day_seventeen;

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
        8 => day_eight(),
        9 => day_nine(),
        10 => day_ten(),
        11 => day_eleven(),
        12 => day_twelve(),
        13 => day_thirteen(),
        14 => day_fourteen(),
        15 => day_fifteen(),
        17 => day_seventeen(),
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
    day_eight();
    day_nine();
    day_ten();
    day_eleven();
    day_twelve();
    day_thirteen();
    day_fourteen();
    day_fifteen();
    day_seventeen();

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

fn day_eight() {
    println!("running day eight...");
    let start = Instant::now();
    day_eight::run_day_08();
    println!("executing day took {} ms", start.elapsed().as_millis());
}

fn day_nine() {
    println!("running day nine...");
    let start = Instant::now();
    day_nine::run_day_09();
    println!("executing day took {} ms", start.elapsed().as_millis());
}

fn day_ten() {
    println!("running day ten...");
    let start = Instant::now();
    day_ten::run_day_10();
    println!("executing day took {} ms", start.elapsed().as_millis());
}

fn day_eleven() {
    println!("running day eleven...");
    let start = Instant::now();
    day_eleven::run_day_11();
    println!("executing day took {} ms", start.elapsed().as_millis());
}

fn day_twelve() {
    println!("running day twelve...");
    let start = Instant::now();
    day_twelve::run_day_12();
    println!("executing day took {} ms", start.elapsed().as_millis());
}

fn day_thirteen() {
    println!("running day thirteen...");
    let start = Instant::now();
    day_thirteen::run_day_13();
    println!("executing day took {} ms", start.elapsed().as_millis());
}

fn day_fourteen() {
    println!("running day fourteen...");
    let start = Instant::now();
    day_fourteen::run_day_14();
    println!("executing day took {} ms", start.elapsed().as_millis());
}

fn day_fifteen() {
    println!("running day fifteen...");
    let start = Instant::now();
    day_fifteen::run_day_15();
    println!("executing day took {} ms", start.elapsed().as_millis());
}

fn day_seventeen() {
    println!("running day seventeen...");
    let start = Instant::now();
    day_seventeen::run_day_17();
    println!("executing day took {} ms", start.elapsed().as_millis());
}
