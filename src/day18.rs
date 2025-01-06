use std::fs; 
use pest::Parser;
use pest_derive::Parser;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use rustc_hash::FxHashSet as HashSet;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: isize,
    pos: (isize, isize)
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Parser)]
#[grammar = "day18.pest"]
struct Day18Parser;

fn parse_input() -> Vec::<(isize, isize)> {
    let data = fs::read_to_string("input/day18.txt").expect("Unable to read file");
    let file = Day18Parser::parse(Rule::file, &data)
             .expect("parse failed")
             .next().unwrap();

    let mut bytes = Vec::new();
    for top in file.into_inner() {
        match top.as_rule() {
            Rule::line => {
                let pos = top.into_inner().map(|t| t.as_str().parse::<isize>().unwrap()).collect::<Vec<_>>();
                bytes.push((pos[0], pos[1]));
            }
            Rule::EOI => {}
            _ => unreachable!()
        }
    }
    bytes
}

const H : isize = 71;
const W : isize = 71;

fn part1(bytes: &[(isize, isize)]) -> Option<isize> {
    let set = HashSet::from_iter(bytes.iter());
    let mut visited = HashSet::default();

    let mut heap = BinaryHeap::new();
    heap.push(State { cost: 0, pos: (0,0)});
    while let Some(State {cost, pos}) = heap.pop() {
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        if pos == (H - 1, W - 1) {
            return Some(cost);
        }
        for e in [
            (pos.0 + 1, pos.1),
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 - 1)].iter().filter(|(r, c)| *r >= 0 && *c >= 0 && *c < W && *r < H) {
                if !set.contains(e) {
                    heap.push(State {cost: cost + 1, pos: *e});
                }
        }
    }
    None

}

fn part2(bytes: &[(isize, isize)]) -> (isize, isize) {
    let mut left = 0;
    let mut right = bytes.len();
    loop {
        let mid = (right + left) / 2;
        if part1(&bytes[0..=mid]).is_none() {
            if part1(&bytes[0..=mid-1]).is_some() {
                return bytes[mid]
            }
            right = mid;
        } else {
            left = mid;
        }
    }
}

pub fn day18() -> (isize, (isize, isize)) {
    let bytes = parse_input();
    (part1(&bytes[0..1024]).unwrap(), part2(&bytes))
}
