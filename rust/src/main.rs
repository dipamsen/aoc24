use std::fs;

pub mod day01;

fn main() {
    println!("Hello, world!");

    let day = 1;
    let run = day01::run;
    let filename = format!("../inputs/day{:02}.txt", day);

    let (a, b) = run(&fs::read_to_string(filename).expect("Can't read input file"));

    let start = std::time::Instant::now();
    println!("Day {day}: {} {}", a, b);

    let elapsed = start.elapsed();

    println!("Elapsed: {}ns", elapsed.as_nanos());
}
