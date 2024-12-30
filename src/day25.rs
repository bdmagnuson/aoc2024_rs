use std::fs; 
use pest::Parser;
use pest_derive::Parser;
use ndarray::{Array,Array2};
use itertools::Itertools;

#[derive(Parser)]
#[grammar = "day25.pest"]
struct Day25Parser;

#[derive(PartialEq, Eq)]
enum Spot {
    Dot,
    Hash
}

fn parse_input() -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let data = fs::read_to_string("input/day25.txt").expect("Diable to read file");
    let top = Day25Parser::parse(Rule::top, &data)
             .expect("parse failed")
             .next().unwrap();


    let mut keys = Vec::default();
    let mut locks = Vec::default();
    for elem in top.into_inner() {
        match elem.as_rule() {
            Rule::grid => {
                let grid = elem.into_inner()
                              .flat_map(|l| {
                                  l.into_inner().map(|s| {
                                      match s.as_rule() {
                                          Rule::dot => Spot::Dot,
                                          Rule::hash => Spot::Hash,
                                          _ => unreachable!()
                                      }
                                  }).collect::<Vec<_>>()
                              }).collect::<Vec<_>>();
                let arr = Array::from_shape_vec((7,5), grid).unwrap();
                let arr_transpose = arr.t();
                if arr_transpose[[0,0]] == Spot::Dot {
                    let vals =
                        arr_transpose.rows()
                                     .into_iter()
                                     .map(|r| 6 - r.iter().take_while(|&v| *v == Spot::Dot).count())
                                     .collect::<Vec<_>>();
                    keys.push(vals);
                } else {
                    let vals =
                        arr_transpose.rows()
                                     .into_iter()
                                     .map(|r| r.iter().take_while(|&v| *v == Spot::Hash).count() - 1)
                                     .collect::<Vec<_>>();
                    locks.push(vals);
                }
            }
            _ => unreachable!()
        }
    }
    (keys, locks)
}

fn part1(locks: &Vec<Vec<usize>>, keys: &Vec<Vec<usize>>) -> u32 {
    let mut compat = 0;
    for (l, k) in locks.iter().cartesian_product(keys.iter()) {
        if l.iter().zip(k.iter()).map(|(a,b)| a + b).all(|v| v <= 5) {
            compat += 1;
        }
    }
    compat
}

pub fn day25() -> (u32, u32) {
    let (keys, locks) = parse_input();
    (part1(&locks, &keys),0)
}

