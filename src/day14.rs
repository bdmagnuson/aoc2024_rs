use std::fs; 
use pest::Parser;
use pest_derive::Parser;
use std::cmp::Ordering::{Less, Greater};
use rustc_hash::{FxHashSet as HashSet };

#[derive(Parser)]
#[grammar = "day14.pest"]
struct Day14Parser;

#[derive(Clone,Debug)]
struct Robot {
    px : i32,
    py : i32,
    vx : i32,
    vy : i32
}

fn parse_input() -> Vec<Robot> {
    let data = fs::read_to_string("input/day14.txt").expect("Unable to read file");
    let file = Day14Parser::parse(Rule::file, &data)
             .expect("parse failed")
             .next().unwrap();

    let mut robots = Vec::new();
    for top in file.into_inner() {
        match top.as_rule() {
            Rule::line => {
                let v = top.into_inner().map(|s| s.as_str().parse::<i32>().unwrap()).collect::<Vec<_>>();
                robots.push( Robot { px : v[0], py : v[1], vx : v[2], vy : v[3] });
            }
            Rule::EOI => {}
            _ => unreachable!()
        }
    }
    robots
}

const DIM_X : i32 = 101;
const DIM_Y : i32 = 103;

fn _display(robots : &Vec<Robot>) {
    let set = HashSet::from_iter(robots.iter().map(|r| (r.px, r.py)));
    for y in 0..103 {
        for x in 0..101 {
            print!("{}", if set.contains(&(x, y)) {"x"} else {"."});
        }
        println!("");
    }
}

fn part1(robots : &mut Vec<Robot>) -> i32 {
    for r in robots.iter_mut() {
        r.px = (r.px + r.vx * 100).rem_euclid(DIM_X);
        r.py = (r.py + r.vy * 100).rem_euclid(DIM_Y);
    }
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for r in robots.iter() {
        match (r.px.cmp(&(DIM_X / 2)), r.py.cmp(&(DIM_Y / 2))) {
            (Less,   Less) => { q1 += 1; }
            (Less,  Greater) => { q2 += 1; }
            (Greater,  Less) => { q3 += 1; }
            (Greater, Greater) => { q4 += 1; }
            _ => {}
        }
    }
    q1 * q2 * q3 * q4
}


fn _part2(robots : &mut Vec<Robot>, iter : i32) -> i32 {
    for r in robots.iter_mut() {
        r.px = (r.px + r.vx * iter).rem_euclid(DIM_X);
        r.py = (r.py + r.vy * iter).rem_euclid(DIM_Y);
    }
    _display(robots);
    0
}


pub fn day14() -> (i32, i32) {
    let robots = parse_input();
    /*
    for i in (332..100000).step_by(101) {
        println!("{}", i);
        part2(&mut robots.clone(), i);
    }
    */
    // part2 determined by noticing a cycle around 101 and iterating until i found it visually ¯\_(ツ)_/¯
    (part1(&mut robots.clone()), 6493)
}

