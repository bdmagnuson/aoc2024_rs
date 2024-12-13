use std::fs; 
use pest::Parser;
use pest_derive::Parser;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::{has_path_connecting, all_simple_paths};
use rustc_hash::{FxHashSet as HashSet, FxHashMap as HashMap };
use itertools::Itertools;

#[derive(Parser)]
#[grammar = "day10.pest"]
struct Day10Parser;

type Blah = DiGraph<(), ()>;
type NodeMap = HashMap<(i32, i32), NodeIndex>;

fn node_idx (m : &NodeMap, (r, c): (i32, i32)) -> NodeIndex {
    *m.get(&(r, c)).unwrap()
}

fn parse_input() -> (i32, i32) {
    let data = fs::read_to_string("input/day10.txt").expect("Diable to read file");
    let file = Day10Parser::parse(Rule::file, &data)
             .expect("parse failed")
             .next().unwrap();

    let mut map : [HashSet<(i32,i32)>; 10] = core::array::from_fn(|_| HashSet::default());
    let mut gr : Blah = Blah::default();

    let mut max_r = 0;
    let mut max_c = 0;
    for (r, line) in file.into_inner().enumerate() {
        max_r = r;
        match line.as_rule() {
            Rule::line => {
                for (c, ch) in line.into_inner().enumerate() {
                    if c > max_c {
                        max_c = c;
                    }
                    let height = ch.as_str().parse::<usize>().unwrap();
                    map[height].insert((r as i32, c as i32));
                }
            }
            Rule::EOI => {}
            _ => unreachable!()
        }
    }
    let rows : i32 = max_r as i32 + 1;
    let cols : i32 = max_c as i32 + 1;

    let mut node_map : NodeMap = HashMap::default();
    for p in (0..rows).cartesian_product(0..cols) {
        node_map.insert(p, gr.add_node(()));
    }


    for h in 0..9 {
        for (r, c) in map[h].iter() {
            for p in [(*r, c + 1),
                      (*r, c - 1),
                      (r + 1, *c),
                      (r - 1, *c)] {
                if map[h + 1].contains(&p) {
                    gr.add_edge(node_idx(&node_map, (*r, *c)), node_idx(&node_map, p), ());
                }
            }
        }
    }

    let mut part1 : i32 = 0;
    for (zero, nine) in map[0].iter().cartesian_product(map[9].iter()) {
        if has_path_connecting(&gr, node_idx(&node_map, *zero), node_idx(&node_map, *nine), None) {
            part1 += 1;
        }
    }

    let mut part2 : i32 = 0;
    for (zero, nine) in map[0].iter().cartesian_product(map[9].iter()) {
        part2 += all_simple_paths::<Vec<_>, _>(&gr, node_idx(&node_map, *zero), node_idx(&node_map, *nine), 0, None).collect::<Vec<_>>().len() as i32;
    }
    (part1, part2)
}

pub fn day10() -> (i32, i32) {
    parse_input()
}
