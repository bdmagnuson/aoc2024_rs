use std::fs; 
use regex::Regex;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "day03.pest"]
struct Day03Parser;

fn part1() -> i32 {
    let data = fs::read_to_string("input/day03.txt").expect("Unable to read file");
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let muls : Vec<(i32, i32)> = re.captures_iter(&data).map(|caps| {
        let (_, [l,r]) = caps.extract();
        (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap())
    }).collect();
    muls.iter().fold(0, |acc, (l,r)| acc + l * r)
}

fn part2() -> i32 {
    let data = fs::read_to_string("input/day03.txt").expect("Unable to read file");
    let file = Day03Parser::parse(Rule::file, &data)
             .expect("parse failed")
             .next().unwrap();

    let mut include = true;
    let mut sum = 0;
    for record in file.into_inner() {
        match record.as_rule() {
            Rule::on => {
                include = true;
            }
            Rule::off => {
                include = false;
            }
            Rule::mul => {
                if include {
                    let terms : Vec<i32> = record.into_inner().map(|f| f.as_str().parse::<i32>().unwrap()).collect();
                    sum += terms[0] * terms[1];
                }
            }
            Rule::other => {}
            Rule::EOI => {}
            _ => unreachable!()
        }
    }
    sum
}

pub fn day03() -> (i32, i32) {
    (part1(), part2())
}
