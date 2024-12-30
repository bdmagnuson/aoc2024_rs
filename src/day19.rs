use std::fs; 
use regex::Regex;
use pest::Parser;
use pest_derive::Parser;
use memoize::memoize;

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

#[memoize]
fn all_combs(pattern: String, towels: Vec<String>) -> Option<u64> {
    let sum = towels.iter().filter_map(|t| {
        match pattern.strip_prefix(t) {
            Some("") => {
                Some(1)
            }
            Some(s) => {
                all_combs(s.to_string(), towels.clone())
            }
            None => {
                None
            }
        }
    }).collect::<Vec<_>>().iter().sum();
    Some(sum)

}

fn part1(towels: &Vec::<String>, patterns: &Vec<String>) -> u64 {
    let re = Regex::new(&format!("^({})+$", towels.join("|"))).unwrap();
    let mut possible = 0;
    for p in patterns {
        if re.is_match(p) {
            possible += 1;
        }
    }
    possible
}

fn part2(towels: &Vec::<String>, patterns: &Vec<String>) -> u64 {
    patterns.iter().filter_map(|p| all_combs(p.clone(), towels.clone())).collect::<Vec<_>>().iter().sum()
}

pub fn day19() -> (u64, u64) {
    let (towels, patterns) = parse_input();
    (part1(&towels, &patterns), part2(&towels, &patterns))
}

// 784859102 Low
