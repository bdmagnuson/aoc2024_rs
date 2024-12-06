use std::fs; 
use pest::Parser;
use pest_derive::Parser;
use std::collections::{HashMap, HashSet};

#[derive(Parser)]
#[grammar = "day06.pest"]
struct Day06Parser;

type Pt = (i32, i32);

#[derive(Hash,Eq,PartialEq,Copy,Clone)]
enum Content {
    Blockage,
    Empty
}

#[derive(Hash,Eq,PartialEq,Copy,Clone)]
enum Dir {
    North,
    South,
    East,
    West
}

enum Path {
    Exit(HashSet<Pt>),
    Loop
}

type Lab = HashMap::<Pt,Content>;

fn parse_input() -> (Lab, (Pt, Dir)) {
    let data = fs::read_to_string("input/day06.txt").expect("Unable to read file");
    let file = Day06Parser::parse(Rule::file, &data)
             .expect("parse failed")
             .next().unwrap();

    let mut map = HashMap::new();
    let mut start_loc = (0, 0);
    let mut start_dir = Dir::North;

    for (r, line) in file.into_inner().enumerate() {
        match line.as_rule() {
            Rule::line => {
                for (c, pt) in line.into_inner().enumerate() {
                    let loc = (r as i32, c as i32);
                    match pt.as_rule() {
                        Rule::space => {
                            map.insert(loc, Content::Empty);
                        }
                        Rule::blockage => {
                            map.insert(loc, Content::Blockage);
                        }
                        Rule::guard => {
                            map.insert(loc, Content::Empty);
                            start_loc = loc;
                            start_dir =
                                match pt.into_inner().next().unwrap().as_rule() {
                                    Rule::east => Dir::East,
                                    Rule::west => Dir::West,
                                    Rule::north => Dir::North,
                                    Rule::south => Dir::South,
                                    _ => unreachable!()
                                }
                        }
                        _ => unreachable!()
                    }
                }
            }
            Rule::EOI => {}
            _ => unreachable!()
        }

    }
    (map, (start_loc, start_dir))
}


fn path_len(lab: &Lab, start_pos: Pt, start_dir : Dir) -> Path {
    let mut pos = start_pos;
    let mut dir = start_dir;
    let mut path = HashSet::with_capacity(5000);
    loop {
        if path.contains(&(pos, dir)) {
            return Path::Loop;
        }
        path.insert((pos, dir));
        let new_pos = match dir {
            Dir::North => { (pos.0 - 1, pos.1) }
            Dir::South => { (pos.0 + 1, pos.1) }
            Dir::West  => { (pos.0, pos.1 - 1) }
            Dir::East  => { (pos.0, pos.1 + 1) }
        };
        if let Some(elem) = lab.get(&new_pos) {
            match elem {
                Content::Empty => {
                    pos = new_pos;
                }
                Content::Blockage => {
                    dir = match dir {
                        Dir::North => { Dir::East }
                        Dir::South => { Dir::West }
                        Dir::West  => { Dir::North }
                        Dir::East  => { Dir::South }
                    }
                }
            }
        } else {
            break;
        }
    }
    Path::Exit(HashSet::from_iter(path.iter().map(|(p,_)| *p)))
}

fn part1(lab: &Lab, start_pos: Pt, start_dir : Dir) -> i32 {
    let Path::Exit(path) = path_len(lab, start_pos, start_dir) else { panic!("bad");};
    path.len() as i32
}

fn part2(lab: &Lab, start_pos: Pt, start_dir : Dir) -> i32 {
    let Path::Exit(normal_path) = path_len(lab, start_pos, start_dir) else {panic!("bad")};
    let locs : Vec<&Pt> = lab.iter().filter(|(_, v)| **v == Content::Empty).map(|(k, _)| k).collect();
    let mut loops = 0;
    let mut test_lab = lab.clone();
    for loc in locs {
        if !normal_path.contains(loc) {
            continue;
        }
        if *loc == start_pos {
            continue;
        }
        test_lab.insert(*loc, Content::Blockage);
        match path_len(&test_lab, start_pos, start_dir) {
            Path::Exit(_) => { }
            Path::Loop => {loops += 1;}
        }
        test_lab.insert(*loc, Content::Empty);
    }
    loops
}

pub fn day06() -> (i32, i32) {
    let (lab, (start_pos, start_dir)) = parse_input();
    (part1(&lab, start_pos, start_dir), part2(&lab, start_pos, start_dir))
}
