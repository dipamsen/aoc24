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
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
/* insert-pub-mod */

fn main() {
    let day = env::args().last().unwrap().parse::<i32>().unwrap();
    let filename = format!("../inputs/day{:02}.txt", day);

    let input = fs::read_to_string(filename)
        .expect("Can't read input file")
        .replace("\r\n", "\n");
    let input = input.trim();

    match day {
        1 => println!("Day {:02}: {:?}", day, day01::run(input)),
        2 => println!("Day {:02}: {:?}", day, day02::run(input)),
        3 => println!("Day {:02}: {:?}", day, day03::run(input)),
        4 => println!("Day {:02}: {:?}", day, day04::run(input)),
        5 => println!("Day {:02}: {:?}", day, day05::run(input)),
        6 => println!("Day {:02}: {:?}", day, day06::run(input)),
        7 => println!("Day {:02}: {:?}", day, day07::run(input)),
        8 => println!("Day {:02}: {:?}", day, day08::run(input)),
        9 => println!("Day {:02}: {:?}", day, day09::run(&input)),
        10 => println!("Day {:02}: {:?}", day, day10::run(&input)),
        11 => println!("Day {:02}: {:?}", day, day11::run(&input)),
        12 => println!("Day {:02}: {:?}", day, day12::run(&input)),
        13 => println!("Day {:02}: {:?}", day, day13::run(&input)),
        14 => println!("Day {:02}: {:?}", day, day14::run(&input)),
        15 => println!("Day {:02}: {:?}", day, day15::run(&input)),
		16 => println!("Day {:02}: {:?}", day, day16::run(&input)),
		17 => println!("Day {:02}: {:?}", day, day17::run(&input)),
		/* insert-run */
        _ => panic!("Day not implemented"),
    };
}
