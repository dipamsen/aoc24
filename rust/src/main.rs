use std::fs;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

fn main() {
    let day = 5;
    let run = day05::run;
    let filename = format!("../inputs/day{:02}.txt", day);

    let input = fs::read_to_string(filename)
        .expect("Can't read input file")
        .replace("\r\n", "\n");

    let (a, b) = run(&input);

    let start = std::time::Instant::now();
    println!("Day {day}: {} {}", a, b);

    let elapsed = start.elapsed();

    println!("Elapsed: {}ns", elapsed.as_nanos());
}
