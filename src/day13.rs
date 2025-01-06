use std::fs; 
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "day13.pest"]
struct Day13Parser;

fn parse_input() -> Vec<Vec<f64>> {
    let data = fs::read_to_string("input/day13.txt").expect("Unable to read file");
    let file = Day13Parser::parse(Rule::file, &data)
             .expect("parse failed")
             .next().unwrap();

    let mut machines = Vec::new();
    for top in file.into_inner() {
        match top.as_rule() {
            Rule::line => {
                machines.push(top.into_inner().map(|s| {let v = s.as_str().parse::<u32>().unwrap(); v as f64 }).collect::<Vec<_>>());
            }
            Rule::EOI => {}
            _ => unreachable!()
        }
    }
    machines
}

fn cost(xa: f64, ya: f64, xb: f64, yb: f64, x: f64, y: f64) -> Option<i64> {
    let pb = (y * xa - x * ya) / (yb * xa - xb * ya);
    let pa = (x - pb * xb) / xa;

    if (pa.fract() == 0.0) && (pb.fract() == 0.0) {
        Some(pa as i64 * 3 + pb as i64)
    } else {
        None
    }

}


fn part1() -> i64 {
    let input = parse_input();
    let mut total = 0;
    for m in input.iter() {
        if let Some(n) = cost(m[0], m[1], m[2], m[3], m[4], m[5]) {
            total += n;
        }
    }
    total
}

fn part2() -> i64 {
    let input = parse_input();
    let mut total = 0;
    for m in input.iter() {
        if let Some(n) = cost(m[0], m[1], m[2], m[3], m[4] + 10000000000000.0, m[5] + 10000000000000.0) {
            total += n;
        }
    }
    total
}


pub fn day13() -> (i64, i64) {
    (part1(),part2())
}

