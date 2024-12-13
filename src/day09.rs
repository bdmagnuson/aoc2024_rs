use std::fs; 
use pest::Parser;
use pest_derive::Parser;
use std::collections::{VecDeque};
use itertools::Itertools;
use std::iter::repeat;
use rustc_hash::{FxHashSet as HashSet};

#[derive(Parser)]
#[grammar = "day09.pest"]
struct Day09Parser;

type Disk = VecDeque<Loc>;
type Disk2 = VecDeque<Loc2>;

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
enum Loc {
    Used(i64),
    Empty
}

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
enum Loc2 {
    Used((i64, i64)),
    Empty(i64)
}

fn parse_input2() -> Disk2 {
    let data = fs::read_to_string("input/day09.txt").expect("Unable to read file");
    let file = Day09Parser::parse(Rule::file, &data)
             .expect("parse failed")
             .next().unwrap();

    let mut id = 0;
    let mut disk : Disk2 = VecDeque::new();
    for top in file.into_inner() {
        match top.as_rule() {
            Rule::input => {
                for mut pair in &top.into_inner().map(|d| d.as_str().parse::<i64>().unwrap()).chunks(2) {
                    if let Some(s) = pair.next() {
                        disk.push_back(Loc2::Used((id, s)));
                        id += 1;
                    }
                    if let Some(z) = pair.next() {
                        disk.push_back(Loc2::Empty(z))
                    }
                }
            }
            Rule::EOI => {}
            _ => unreachable!()
        }
    }
    disk
}


fn parse_input1() -> Disk {
    let data = fs::read_to_string("input/day09.txt").expect("Unable to read file");
    let file = Day09Parser::parse(Rule::file, &data)
             .expect("parse failed")
             .next().unwrap();

    let mut id = 0;
    let mut disk : VecDeque<Loc> = VecDeque::new();
    for top in file.into_inner() {
        match top.as_rule() {
            Rule::input => {
                for mut pair in &top.into_inner().map(|d| d.as_str().parse::<i64>().unwrap()).chunks(2) {
                    if let Some(s) = pair.next() {
                        disk.append(&mut VecDeque::from_iter(repeat(Loc::Used(id)).take(s as usize)));
                        id += 1;
                    }
                    if let Some(z) = pair.next() {
                        disk.append(&mut VecDeque::from_iter(repeat(Loc::Empty).take(z as usize)));
                    }
                }
            }
            Rule::EOI => {}
            _ => unreachable!()
        }
    }
    disk
}

fn part1(mut input: Disk) -> i64 {
    let mut defrag : Disk = VecDeque::new();
    while let Some(head) = input.pop_front() {
       match head {
           Loc::Empty => {
               while let Some(tail) = input.pop_back() {
                   match tail {
                       Loc::Empty => {
                           continue;
                       }
                       Loc::Used(_) => {
                           defrag.push_back(tail);
                           break;
                       }
                   }
                }
            }
           Loc::Used(_) => {
               defrag.push_back(head);
           }
       }
    }

    fn id (loc : &Loc) -> i64 {
        match loc {
            Loc::Empty => {unreachable!();}
            Loc::Used(i) => *i
        }
    }
    defrag.iter().enumerate().fold(0, |acc, (idx, pos)| acc + (idx as i64) * id(pos))
}

fn part2(input: Disk2) -> i64 {
    let mut defrag : Disk2 = VecDeque::new();
    let mut moved : HashSet<i64> = HashSet::default();

    let blocks = input.iter()
                      .filter(|m| matches!(m, Loc2::Used(_)))
                      .map(|u| { let Loc2::Used(s) = u else {panic!("wtf2");}; s }).collect::<VecDeque<_>>();

    for b in input.iter() {
        match b {
            Loc2::Used((id, s)) => {
                if moved.contains(id) {
                    defrag.push_back(Loc2::Empty(*s));
                } else {
                    moved.insert(*id);
                    defrag.push_back(*b);
                }
            }
            Loc2::Empty(s) => {
                let mut space = *s;
                loop {
                    let mut no_moves = true;
                    for idx in (0..blocks.len()).rev() {
                        if moved.contains(&(idx as i64)) {
                            continue;
                        }
                        if blocks[idx].1 > space {
                            continue;
                        }
                        defrag.push_back(Loc2::Used(*blocks[idx]));
                        space -= blocks[idx].1;
                        moved.insert(idx as i64);
                        no_moves = false;
                    }
                    if no_moves {
                        break;
                    }
                }
                defrag.push_back(Loc2::Empty(space));
            }
        }
    }

    let mut checksum = 0;
    let mut idx = 0;
    for b in defrag.iter() {
        match b {
            Loc2::Empty(sz) => {
                idx += sz;
            }
            Loc2::Used((id, sz)) => {
                checksum += (idx..idx+sz).map(|i| i * id).sum::<i64>();
                idx += sz;
           }
        }
    }
    checksum
}

pub fn day09() -> (i64, i64) {
    let input1 = parse_input1();
    let input2 = parse_input2();
    (part1(input1), part2(input2))
}

// 10816322049364 H
