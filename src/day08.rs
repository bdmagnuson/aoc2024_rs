use std::fs; 
use pest::Parser;
use pest_derive::Parser;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet };
use itertools::Itertools;


#[derive(Parser)]
#[grammar = "day08.pest"]
struct Day08Parser;

type Pt = (i32, i32);
type StationMap = HashMap<char, HashSet<Pt>>;
type Input = (Pt, StationMap);

fn parse_input() -> Input {
    let data = fs::read_to_string("input/day08.txt").expect("Unable to read file");
    let file = Day08Parser::parse(Rule::file, &data)
             .expect("parse failed")
             .next().unwrap();

    let mut max_r = 0;
    let mut max_c = 0;
    let mut stations : StationMap = HashMap::default();
    for (r, top) in file.into_inner().enumerate() {
        match top.as_rule() {
            Rule::line => {
                max_r = r;
                for (c, ch) in top.into_inner().enumerate() {
                    if c > max_c {
                        max_c = c;
                    }
                    let pos = (r as i32, c as i32);
                    match ch.as_rule() {
                        Rule::empty => {},
                        Rule::station => { stations.entry(ch.as_str().chars().next().unwrap())
                                                   .and_modify(|s : &mut HashSet<Pt>| { s.insert(pos);})
                                                   .or_insert(HashSet::from_iter([pos])); }
                        _ => unreachable!()
                    }
                }
            }
            Rule::EOI => {}
            _ => unreachable!()
        }
    }
    ((max_r as i32, max_c as i32), stations)
}


fn part1(((max_r, max_c), map): &Input) -> i32 {
    let mut antinodes : HashSet<Pt>  = HashSet::default();
    map.iter().for_each(|(_, v) : (_, &HashSet<Pt>)| {
        for pair in v.iter().combinations(2) {
           let (r1, c1) = pair[0];
           let (r2, c2) = pair[1];
           let (node1, node2) =
               if r1 > r2 {
                   let (dr, dc) = ((r1 - r2), (c1 - c2));
                   let node1 = (r1 + dr, c1 + dc);
                   let node2 = (r2 - dr, c2 - dc);
                   (node1, node2)
               } else {
                   let (dr, dc) = ((r2 - r1), (c2 - c1));
                   let node1 = (r2 + dr, c2 + dc);
                   let node2 = (r1 - dr, c1 - dc);
                   (node1, node2)
               };
            if node1.0 >= 0 && node1.0 <= *max_r && node1.1 >= 0 && node1.1 <= *max_c {
                antinodes.insert(node1);
            }
            if node2.0 >= 0 && node2.0 <= *max_r && node2.1 >= 0 && node2.1 <= *max_c {
                antinodes.insert(node2);
            }
        }
    });
    antinodes.len() as i32
}

fn part2(((max_r, max_c), map): &Input) -> i32 {
    let mut antinodes : HashSet<Pt>  = HashSet::default();
    map.iter().for_each(|(_, v) : (_, &HashSet<Pt>)| {
        for pair in v.iter().combinations(2) {
           let (r1, c1) = pair[0];
           let (r2, c2) = pair[1];
           let (mut n1, mut n2, step) = 
               if r1 > r2 {
                   let (dr, dc) = ((r1 - r2), (c1 - c2));
                   let n1 = (r1 + dr, c1 + dc);
                   let n2 = (r2 - dr, c2 - dc);
                   (n1, n2, (dr, dc))
               } else {
                   let (dr, dc) = ((r2 - r1), (c2 - c1));
                   let n1 = (r2 + dr, c2 + dc);
                   let n2 = (r1 - dr, c1 - dc);
                   (n1, n2, (dr, dc))
               };
           let mut new_nodes = vec![(*r1, *c1), (*r2, *c2)];
           while n1.0 >= 0 && n1.0 <= *max_r && n1.1 >= 0 && n1.1 <= *max_c {
               new_nodes.push(n1);
               n1 = (n1.0 + step.0, n1.1 + step.1);
           }
           while n2.0 >= 0 && n2.0 <= *max_r && n2.1 >= 0 && n2.1 <= *max_c {
               new_nodes.push(n2);
               n2 = (n2.0 - step.0, n2.1 - step.1);
           }
           new_nodes.iter().for_each(|n| { antinodes.insert(*n); });
        }
    });
    antinodes.len() as i32
}

pub fn day08() -> (i32, i32) {
    let input = parse_input();
    (part1(&input), part2(&input))
}
