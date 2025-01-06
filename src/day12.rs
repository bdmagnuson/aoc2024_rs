use std::fs; 
use pest::Parser;
use pest_derive::Parser;
use rustc_hash::{FxHashSet as HashSet };
use std::cmp::Ordering::{Less, Greater};


#[derive(Parser)]
#[grammar = "day12.pest"]
struct Day12Parser;

#[derive(Debug,Clone,PartialEq,Eq)]
enum Dir {
    Right,
    Left,
    Up,
    Down
}

fn parse_input() -> (i32, i32) {
    let data = fs::read_to_string("input/day12.txt").expect("Unable to read file");
    let file = Day12Parser::parse(Rule::file, &data)
             .expect("parse failed")
             .next().unwrap();

    let mut plots = Vec::new();
    for (r, top) in file.into_inner().enumerate() {
        match top.as_rule() {
            Rule::line => {
                for (c, ch) in top.into_inner().enumerate() {
                    let pos = (r as i32, c as i32);
                    plots.push((pos, ch.as_str().chars().next().unwrap()))
                }
            }
            Rule::EOI => {}
            _ => unreachable!()
        }
    }

    let mut regions : Vec<(char, HashSet<(i32, i32)>)> = Vec::new();
    for ((r, c), ch) in plots.iter() {
        let mut new_region = true;
        let mut union_region = HashSet::default();
        for (dr, dc) in [
            ( 0,  1),
            ( 0, -1),
            ( 1,  0),
            (-1,  0)
        ] {
            let ro : i32 = r + dr;
            let co : i32 = c + dc;
            regions.iter_mut().for_each(|(ch_, ref mut rr)| {
                if ch == ch_ && rr.contains(&(ro, co)) {
                    rr.insert((*r,*c));
                    union_region.extend(rr.iter());
                    new_region = false;
                }
            });
        }
        if new_region {
            regions.push((*ch, HashSet::from_iter([(*r,*c)])));
        } else {
            regions = regions.into_iter().filter(|(ch_, rr)| ch != ch_ || !rr.contains(&(*r, *c))).collect::<Vec<_>>();
            regions.push((*ch, union_region));
        }
    }

    let mut part1 = 0;
    for (_, region) in regions.iter() {
        let mut perim = 0;
        for (r, c) in region.iter() {
            for (dr, dc) in [
                ( 0,  1),
                ( 0, -1),
                ( 1,  0),
                (-1,  0)
            ] {
                if !region.contains(&(r + dr, c + dc)) {
                    perim += 1;
                }
            }
        }
        part1 += perim * region.len();
    }

    let mut part2 = 0;
    for (_, region) in regions.iter() {
        let mut edges : Vec<(Dir, HashSet<(i32, i32)>)> = Vec::new();

        let mut locs = region.iter().collect::<Vec<_>>();
        locs.sort_by(|(r1, c1), (r2, c2)| {
            match r1.cmp(r2) {
                Less => Less,
                Greater => Greater,
                _ => c1.cmp(c2)
            }
        });

        for (r, c) in locs.iter() {
            // Right
            if !region.contains(&(*r, c + 1)) {
                let mut new_edge = true;
                for (dir, s) in edges.iter_mut() {
                    if *dir != Dir::Right {
                        continue;
                    } else if s.contains(&(r - 1, *c)) || s.contains(&(r + 1, *c)) {
                        s.insert((*r, *c));
                        new_edge = false;
                    }
                }
                if new_edge {
                    edges.push((Dir::Right, HashSet::from_iter([(*r, *c)])));
                }
            }

            // Left
            if !region.contains(&(*r, c - 1)) {
                let mut new_edge = true;
                for (dir, s) in edges.iter_mut() {
                    if *dir != Dir::Left {
                        continue;
                    } else if s.contains(&(r - 1, *c)) || s.contains(&(r + 1, *c)) {
                        s.insert((*r, *c));
                        new_edge = false;
                    }
                }
                if new_edge {
                    edges.push((Dir::Left, HashSet::from_iter([(*r, *c)])));
                }
            }
        }

        for (r, c) in locs.iter() {
            // Up
            if !region.contains(&(r - 1, *c)) {
                let mut new_edge = true;
                for (dir, s) in edges.iter_mut() {
                    if *dir != Dir::Up {
                        continue;
                    } else if s.contains(&(*r, c - 1)) || s.contains(&(*r, c + 1)) {
                        s.insert((*r, *c));
                        new_edge = false;
                    }
                }
                if new_edge {
                    edges.push((Dir::Up, HashSet::from_iter([(*r, *c)])));
                }
            }

            // Down
            if !region.contains(&(r + 1, *c)) {
                let mut new_edge = true;
                for (dir, s) in edges.iter_mut() {
                    if *dir != Dir::Down {
                        continue;
                    } else if s.contains(&(*r, c - 1)) || s.contains(&(*r, c + 1)) {
                        s.insert((*r, *c));
                        new_edge = false;
                    }
                }
                if new_edge {
                    edges.push((Dir::Down, HashSet::from_iter([(*r, *c)])));
                }
            }
        }
        part2 += region.len() * edges.len();
    }
    (part1 as i32, part2 as i32)
}


pub fn day12() -> (i32, i32) {
    parse_input()
}

//1115805 H
