use std::fs; 
use regex::Regex;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "day19.pest"]
struct Day19Parser;

fn parse_input() -> (Vec::<String>, Vec<String>) {
    let data = fs::read_to_string("input/day19.txt").expect("Unable to read file");
    let file = Day19Parser::parse(Rule::file, &data)
             .expect("parse failed")
             .next().unwrap();

    let mut towels = Vec::new();
    let mut patterns = Vec::new();
    for top in file.into_inner() {
        match top.as_rule() {
            Rule::towels => {
                towels = top.into_inner().map(|t| t.as_str().to_string()).collect::<Vec<_>>();
            }
            Rule::patterns => {
                patterns = top.into_inner().map(|t| t.as_str().to_string()).collect::<Vec<_>>();
            }
            Rule::EOI => {}
            _ => unreachable!()
        }
    }
    (towels, patterns)
}

fn part1(towels: &Vec::<String>, patterns: &Vec<String>) -> i32 {
    let re = Regex::new(&format!("^({})+$", towels.join("|"))).unwrap();
    let mut possible = 0;
    for p in patterns {
        if re.is_match(p) {
            possible += 1;
        }
    }
    possible
}

fn part2() -> i32 {
    0
}

pub fn day19() -> (i32, i32) {
    let (towels, patterns) = parse_input();
    (part1(&towels, &patterns), part2())
}
