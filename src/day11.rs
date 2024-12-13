//use std::fs; 
//use pest::Parser;
//use pest_derive::Parser;

/*
#[derive(Parser)]
#[grammar = "day10.pest"]
struct Day10Parser;
*/


use memoize::memoize;
#[memoize]
fn split (n : u64, steps: usize) -> u64 {
    let digits = if n == 0 {0} else {n.ilog10() + 1};
    let mid = 10_u64.pow(digits / 2);
    let div =
        if n == 0 {
            vec![1]
        } else if digits % 2 == 0 {
            vec![n / mid, n % mid]
        } else {
            vec![n * 2024]
        };
    if steps == 1 {
        div.len() as u64
    } else {
        div.iter().map(|s| split(*s, steps - 1)).sum()
    }
}

pub fn day11() -> (u64, u64) {
    let input = [872027, 227, 18, 9760, 0, 4, 67716, 9245696];
    let part1 = input.iter().map(|s| split(*s, 25)).sum();
    let part2 = input.iter().map(|s| split(*s, 75)).sum();

    (part1, part2)
}
