use std::fs; 
use std::cmp::Ordering;
use pest::Parser;
use pest_derive::Parser;
use petgraph::graphmap::{DiGraphMap};

#[derive(Parser)]
#[grammar = "day05.pest"]
struct Day05Parser;

type Blah = DiGraphMap<i32, ()>;
type Rules = Vec::<Vec::<i32>>;

fn parse_input() -> (Blah, Rules) {
    let data = fs::read_to_string("input/day05.txt").expect("Diable to read file");
    let file = Day05Parser::parse(Rule::file, &data)
             .expect("parse failed")
             .next().unwrap();

    let mut rules = Vec::new();
    let mut updates = Vec::new();
    for record in file.into_inner() {
        match record.as_rule() {
            Rule::order => {
                let mut pair = record.into_inner();
                let fst = pair.next().unwrap().as_str().parse::<i32>().unwrap();
                let snd = pair.next().unwrap().as_str().parse::<i32>().unwrap();
                rules.push((fst, snd));
            }
            Rule::update => {
                updates.push(record.into_inner().map(|u| u.as_str().parse::<i32>().unwrap()).collect());
            }
            Rule::EOI => {}
            _ => unreachable!()
        }
    }
    let gr = DiGraphMap::from_edges(&rules);
    (gr, updates)
}

fn tails<T:Clone+Copy>(v: &[T]) -> Vec<(T, Vec<T>)> {
    let mut out = Vec::new();
    for i in 0..v.len() - 1 {
        out.push((v[i], v[i+1..].to_vec()));
    }
    out
}

fn part1(gr : &Blah, rules : &Rules ) -> i32 {
    rules.iter().filter_map(|rule| {
        let res = tails(rule).iter().all(|(s, t)| {
            !t.iter().any(|e| DiGraphMap::contains_edge(gr, *e, *s))
        });
        if res {
            Some(rule[rule.len() / 2])
        } else {
            None
        }
    }).collect::<Vec<_>>().iter().sum()
}

fn part2(gr : &Blah, rules : &Rules ) -> i32 {
    let mut bads = rules.clone().into_iter().filter(|rule| {
        let res = tails(rule).iter().any(|(s, t)| {
            t.iter().any(|e| DiGraphMap::contains_edge(gr, *e, *s))
        });
        res
    }).collect::<Vec<_>>();
    bads.iter_mut().for_each(|bb| (*bb).sort_by(|a, b| if DiGraphMap::contains_edge(gr, *a, *b) {Ordering::Greater} else {Ordering::Less}));
    bads.iter().map(|b| b[b.len() / 2]).collect::<Vec<i32>>().iter().sum()
}

pub fn day05() -> (i32, i32) {
    let (gr, rules) = parse_input();
    (part1(&gr, &rules), part2(&gr, &rules))
}
