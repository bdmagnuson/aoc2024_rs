#![feature(sync_unsafe_cell)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

use std::time::Instant;

fn run<T:std::fmt::Debug>(day: i32, f:fn () -> T) {
    let now = Instant::now();
    println!("day{:02}: {:?} {}ms", day, f(), now.elapsed().as_millis());
}


fn main() {
    run(1, day01::day01);
    run(2, day02::day02);
    run(3, day03::day03);
    run(4, day04::day04);
    run(5, day05::day05);
    run(6, day06::day06);
    run(7, day07::day07);
    run(8, day08::day08);
    run(9, day09::day09);
    run(10, day10::day10);
    run(11, day11::day11);
    run(12, day12::day12);
    run(13, day13::day13);
    run(14, day14::day14);
    run(15, day15::day15);
    run(16, day16::day16);
    run(17, day17::day17);
    run(18, day18::day18);
    run(19, day19::day19);
    run(20, day20::day20);
    run(21, day21::day21);
    run(22, day22::day22);
    run(23, day23::day23);
    run(24, day24::day24);
    run(25, day25::day25);
}
