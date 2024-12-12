use std::{env, fs};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
/* insert-pub-mod */

fn main() {
    let day = env::args().last().unwrap().parse::<i32>().unwrap();
    let filename = format!("../inputs/day{:02}.txt", day);

    let input = fs::read_to_string(filename)
        .expect("Can't read input file")
        .replace("\r\n", "\n");
    let input = input.trim();

    print!("Day {:02}: ", day);
    match day {
        1 => println!("{:?}", day01::run(input)),
        2 => println!("{:?}", day02::run(input)),
        3 => println!("{:?}", day03::run(input)),
        4 => println!("{:?}", day04::run(input)),
        5 => println!("{:?}", day05::run(input)),
        6 => println!("{:?}", day06::run(input)),
        7 => println!("{:?}", day07::run(input)),
        8 => println!("{:?}", day08::run(input)),
        9 => println!("{:?}", day09::run(&input)),
		10 => println!("{:?}", day10::run(&input)),
		11 => println!("{:?}", day11::run(&input)),
		12 => println!("{:?}", day12::run(&input)),
		/* insert-run */
        _ => panic!("Day not implemented"),
    };
}
