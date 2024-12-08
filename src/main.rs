
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

use std::time::Instant;

fn main() {
    let now = Instant::now();
    println!("day01: {:?} {}ms", day01::day01(), now.elapsed().as_millis());
    let now = Instant::now();
    println!("day02: {:?} {}ms", day02::day02(), now.elapsed().as_millis());
    let now = Instant::now();
    println!("day03: {:?} {}ms", day03::day03(), now.elapsed().as_millis());
    let now = Instant::now();
    println!("day04: {:?} {}ms", day04::day04(), now.elapsed().as_millis());
    let now = Instant::now();
    println!("day05: {:?} {}ms", day05::day05(), now.elapsed().as_millis());
    let now = Instant::now();
    println!("day06: {:?} {}ms", day06::day06(), now.elapsed().as_millis());
    let now = Instant::now();
    println!("day07: {:?} {}ms", day07::day07(), now.elapsed().as_millis());
    let now = Instant::now();
    println!("day08: {:?} {}ms", day08::day08(), now.elapsed().as_millis());
}
