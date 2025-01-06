use std::fs;
use pest::Parser;
use pest_derive::Parser;
use rustc_hash::{FxHashSet as HashSet, FxHashMap as HashMap};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Parser)]
#[grammar = "day20.pest"]
struct Day20Parser;

type Pt = (isize, isize);

struct Maze {
    start : Pt,
    end : Pt,
    width : usize,
    height : usize,
    walls : HashSet<Pt>,
    spaces : HashSet<Pt>
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: Pt
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


fn parse_input() -> Maze {
    let data = fs::read_to_string("input/day20.txt").expect("Diable to read file");
    let file = Day20Parser::parse(Rule::file, &data)
             .expect("parse failed")
             .next().unwrap();

    let mut walls = HashSet::default();
    let mut spaces = HashSet::default();
    let mut start = (0,0);
    let mut end = (0,0);
    let mut max_r = 0;
    let mut max_c = 0;
    for (r, line) in file.into_inner().enumerate() {
        match line.as_rule() {
            Rule::line => {
                max_r = r;
                for (c, ch) in line.into_inner().enumerate() {
                    if c > max_c {
                        max_c = c;
                    }
                    let pos = (r as isize, c as isize);
                    match ch.as_rule() {
                        Rule::wall  => { walls.insert(pos); }
                        Rule::start => { spaces.insert(pos); start = pos; }
                        Rule::end   => { end = pos; }
                        Rule::space => { spaces.insert(pos); }
                        _ => unreachable!()
                    }
                }
            }
            Rule::EOI => {}
            _ => unreachable!()
        }
    }
    Maze { start, end, walls, spaces, height: max_r + 1, width: max_c + 1}
}

fn cost(maze: &Maze) -> HashMap<Pt, usize> {
    let mut dist = HashMap::default();
    let mut heap = BinaryHeap::new();

    heap.push(State { cost: 0, pos: maze.start });
    dist.insert(maze.start, 0);

    while let Some(State {cost, pos}) = heap.pop() {
        if let Some(s) = dist.get(&pos) {
            if *s < cost {
                continue;
            }
        }
        for e in [
            (pos.0 + 1, pos.1),
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 - 1)].iter().filter(|(r, c)| *r >= 0 && *c >= 0 && *c < maze.width as isize && *r < maze.height as isize) {
                if !maze.walls.contains(e) {
                    let next = State {cost: cost + 1, pos: *e};
                    match dist.get(&next.pos) {
                        None => {
                            dist.insert(next.pos, next.cost);
                            heap.push(next);
                        }
                        Some(s) => {
                            if next.cost < *s {
                                dist.insert(next.pos, next.cost);
                                heap.push(next);
                            }
                        }

                    }
                }
            }
    }
    let best = *dist.get(&maze.end).unwrap();
    for (_, d) in dist.iter_mut() {
        *d = best - *d;
    }
    dist
}

fn eval_cheats(maze: &Maze, range: isize) -> i32 {
    let costs = cost(maze);
    let mut offsets = Vec::default();
    let mut cheats = HashMap::default();
    for x in -range..=range {
        for y in -range+x.abs()..=range-x.abs() {
            offsets.push((x,y));
        }
    }
    for s in maze.spaces.iter() {
        for o in offsets.iter() {
            let cheat_cost = o.0.abs() + o.1.abs();
            let init_cost = costs.get(s).unwrap();
            let warp_pos = (s.0 + o.0, s.1 + o.1);
            if let Some(offset_cost) = costs.get(&warp_pos) {
                let savings = *init_cost as isize - (*offset_cost as isize + cheat_cost);
                if savings >= 100 {
                    cheats.entry(savings).and_modify(|s| *s += 1).or_insert(1);
                }
            }

        }
    }
    let mut part2 = 0;
    for (_,v) in cheats.iter() {
        part2 += v;
    }
    part2
}

pub fn day20() -> (i32, i32) {
    let maze = parse_input();
    (eval_cheats(&maze, 2),eval_cheats(&maze, 20))
}

// 7734136 H
