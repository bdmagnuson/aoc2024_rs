use std::fs; 
use regex::Regex;
use pest::Parser;
use pest_derive::Parser;
use cached::proc_macro::cached;

#[derive(Parser)]
#[grammar = "day19.pest"]
struct Day19Parser;

static mut TOWELS_STATIC : Vec<String> = Vec::new();

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
    unsafe { TOWELS_STATIC = towels.clone(); }
    (towels, patterns)
}


#[cached]
fn all_combs<'a>(pattern: String) -> Option<u64> {
    let sum = unsafe {
        TOWELS_STATIC.iter().filter_map(|t| {
            match pattern.strip_prefix(t) {
                Some("") => {
                    Some(1)
                }
                Some(s) => {
                    all_combs(s.to_string())
                }
                None => {
                    None
                }
            }
        }).collect::<Vec<_>>().iter().sum()
    };
    Some(sum)

}

fn part1(towels: &[String], patterns: &Vec<String>) -> u64 {
    let re = Regex::new(&format!("^({})+$", towels.join("|"))).unwrap();
    let mut possible = 0;
    for p in patterns {
        if re.is_match(p) {
            possible += 1;
        }
    }
    possible
}

fn part2(towels: &[String], patterns: &[String]) -> u64 {
    unsafe {
        patterns.iter().filter_map(|p| all_combs(p.clone())).collect::<Vec<_>>().iter().sum()
    }
}

pub fn day19() -> (u64, u64) {
    let (towels, patterns) = parse_input();
    (part1(&towels, &patterns), part2(&towels, &patterns))
}

