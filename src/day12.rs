use std::fs; 
use pest::Parser;
use pest_derive::Parser;
use rustc_hash::{FxHashSet as HashSet };


#[derive(Parser)]
#[grammar = "day12.pest"]
struct Day12Parser;

fn parse_input() {
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
                    union_region = union_region.union(&rr).map(|r| *r).collect();
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
    println!("{:#?}", part1);
}


fn part1() -> i32 {
    0
}

fn part2() -> i32 {
    0
}

pub fn day12() -> (i32, i32) {
    parse_input();
    (0,0)
}
