use std::fs; 
use pest::Parser;
use pest_derive::Parser;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use rustc_hash::FxHashSet as HashSet;

#[derive(Parser)]
#[grammar = "day16.pest"]
struct Day16Parser;

type Pt = (usize,usize);

#[derive(Hash,PartialEq,Eq,Clone,Copy)]
enum Dir {
    North,
    South,
    East,
    West
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: isize,
    path: Vec<Pt>,
    dir: Dir,
    pos: (usize, usize)
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

fn parse_input() -> (HashSet<Pt>, Pt, Pt) {
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
    (map, start, end)
}



fn doit(map: &HashSet<Pt>, start: &Pt, end: &Pt) -> (u32, u32) {
    let mut visited = HashSet::default();

    let mut heap = BinaryHeap::new();
    let mut min_cost = None;
    let mut all_paths: HashSet<Pt> = HashSet::default();

    heap.push(State { cost: 0, pos: *start, dir: Dir::East, path: vec![*start] } );
    while let Some(State {cost, path, dir, pos}) = heap.pop() {
        visited.insert((pos, dir));
        if let Some(min_cost) = min_cost {
            if cost > min_cost {
                continue;
            }
        }
        if pos == *end {
            min_cost = Some(cost);
            all_paths.extend(path.iter());
            continue;
        }
        let step = match dir {
            Dir::North => ((pos.0 - 1, pos.1), Dir::North, 1),
            Dir::South => ((pos.0 + 1, pos.1), Dir::South, 1),
            Dir::East  => ((pos.0, pos.1 - 1), Dir::East, 1),
            Dir::West  => ((pos.0, pos.1 + 1), Dir::West, 1)
        };
        let mut turns = match dir {
            Dir::North => vec![(pos, Dir::East, 1000),  (pos, Dir::West, 1000)],
            Dir::South => vec![(pos, Dir::East, 1000),  (pos, Dir::West, 1000)],
            Dir::East  => vec![(pos, Dir::South, 1000), (pos, Dir::North, 1000)],
            Dir::West  => vec![(pos, Dir::South, 1000), (pos, Dir::North, 1000)]
        };
        turns.push(step);
        for (ipos, idir, icost) in turns {
            if !visited.contains(&(ipos, idir)) && map.contains(&ipos) {
                let mut ipath = path.clone();
                ipath.push(ipos);
                heap.push(State {cost: cost + icost, pos: ipos, dir: idir, path: ipath});
            }
        }
    }
    (min_cost.unwrap() as u32, all_paths.len() as u32)

}


pub fn day16() -> (u32, u32) {
    let (map, start, end) = parse_input();
    doit(&map, &start, &end)
}
