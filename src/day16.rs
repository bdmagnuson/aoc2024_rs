use std::fs; 
use pest::Parser;
use pest_derive::Parser;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::{dijkstra,all_simple_paths};
use rustc_hash::{FxHashSet as HashSet, FxHashMap as HashMap };
use itertools::Itertools;

#[derive(Parser)]
#[grammar = "day16.pest"]
struct Day16Parser;

type Maze = DiGraph<(), i32>;

#[derive(Hash,Eq,PartialEq,Clone)]
enum Dir {
    North,
    South,
    East,
    West
}

struct Node {
    north: i32,
    south: i32,
    east: i32,
    west: i32
}


fn parse_input() -> (i32, i32) {
    let data = fs::read_to_string("input/day16.txt").expect("Diable to read file");
    let file = Day16Parser::parse(Rule::file, &data)
             .expect("parse failed")
             .next().unwrap();

    let mut map = HashSet::default();
    let mut start = (0,0);
    let mut end = (0,0);

    for (r, line) in file.into_inner().enumerate() {
        match line.as_rule() {
            Rule::line => {
                for (c, ch) in line.into_inner().enumerate() {
                    match ch.as_rule() {
                        Rule::space => { map.insert((r,c)); }
                        Rule::wall => {}
                        Rule::start => {
                            map.insert((r,c));
                            start = (r, c);
                        }
                        Rule::end => {
                            map.insert((r,c));
                            end = (r, c);
                        }
                        _ => unreachable!()
                    }
                }
            }
            Rule::EOI => {}
            _ => unreachable!()
        }
    }

    let mut gr = Maze::default();
    let node_map = HashMap::from_iter(map.iter().flat_map(|p| {
        let n = gr.add_node(());
        let s = gr.add_node(());
        let w = gr.add_node(());
        let e = gr.add_node(());
        gr.add_edge(n, e, 1000);
        gr.add_edge(e, s, 1000);
        gr.add_edge(s, w, 1000);
        gr.add_edge(w, n, 1000);
        gr.add_edge(n, w, 1000);
        gr.add_edge(w, s, 1000);
        gr.add_edge(s, e, 1000);
        gr.add_edge(e, n, 1000);
        [((p, Dir::North), n), ((p, Dir::South), s), ((p, Dir::East), e), ((p, Dir::West), w)]
    }));

    for (((r, c), d), s) in node_map.iter() {
        if *d == Dir::North && map.contains(&(r - 1, c + 0)) {
            gr.add_edge(*s, *node_map.get(&(&(r - 1, *c), Dir::North)).unwrap(), 1);
        }
        if *d == Dir::South && map.contains(&(r + 1, c + 0)) {
            gr.add_edge(*s, *node_map.get(&(&(r + 1, *c), Dir::South)).unwrap(), 1);
        }
        if *d == Dir::East && map.contains(&(r + 0, c + 1)) {
            gr.add_edge(*s, *node_map.get(&(&(*r, c + 1), Dir::East)).unwrap(), 1);
        }
        if *d == Dir::West && map.contains(&(r + 0, c - 1)) {
            gr.add_edge(*s, *node_map.get(&(&(*r, c - 1), Dir::West)).unwrap(), 1);
        }
    }
    let s = node_map.get(&(&start, Dir::East)).unwrap();
    let cost = dijkstra(&gr, *s, None, |e| *e.weight());
    let dest_n = node_map.get(&(&end, Dir::North)).unwrap();
    let dest_s = node_map.get(&(&end, Dir::South)).unwrap();
    let dest_e = node_map.get(&(&end, Dir::East)).unwrap();
    let dest_w = node_map.get(&(&end, Dir::West)).unwrap();
    let part1 = [
        cost.get(dest_n).unwrap(),
        cost.get(dest_s).unwrap(),
        cost.get(dest_e).unwrap(),
        cost.get(dest_w).unwrap()
    ].into_iter().min().unwrap();

    /*

    let mut all_paths : HashSet<NodeIndex> = HashSet::default();
    for path in all_simple_paths::<Vec<_>, _>(&gr, *s, *dest_n, 0, None) {
        let pp = path.iter().map(|i| *i).collect::<Vec<_>>();
        let s = shortest_path(&gr, &pp, *part1);
        println!("{:?}", s);
        if s.is_some() {
            all_paths.extend(s.unwrap().clone());
        }
    }
    */


    (*part1,0)
}

fn shortest_path<'a> (gr: &Maze, path: &'a[NodeIndex], cost: i32) -> Option<HashSet<NodeIndex>>
{
    let mut total =0;

    for (n1, n2) in path.iter().zip(path.iter().next()) {
        total += *gr.edge_weight(gr.find_edge(*n1, *n2).unwrap()).unwrap();
        if total > cost {
            return None;
        }
    }
    if total == cost {
        Some(HashSet::from_iter(path.iter().map(|p| *p)))
    } else {
        None
    }


}


pub fn day16() -> (i32, i32) {
    parse_input()
}
