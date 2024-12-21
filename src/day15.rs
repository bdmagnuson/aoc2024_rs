use std::fs; 
use pest::Parser;
use pest_derive::Parser;
use ndarray::{Array, Array2, s};

#[derive(Parser)]
#[grammar = "day15.pest"]
struct Day15Parser;

#[derive(Debug,PartialEq,Eq,Clone)]
enum Obj {
    Wall,
    Box,
    Bot,
    Empty
}

#[derive(Debug,PartialEq,Eq,Clone)]
enum Dir {
    Left,
    Right,
    Up,
    Down
}

fn parse_input() -> (Array2<Obj>, Vec<Dir>, (usize, usize)) {
    let data = fs::read_to_string("input/day15.txt").expect("Unable to read file");
    let file = Day15Parser::parse(Rule::file, &data)
             .expect("parse failed")
             .next().unwrap();


    let mut moves = Vec::new();
    let mut map =  Vec::new();
    let mut start = (0,0);
    for top in file.into_inner() {
        match top.as_rule() {
            Rule::map => {
                for (r, line) in top.into_inner().enumerate() {
                    let mut l = Vec::new();
                    for (c, ch) in line.into_inner().enumerate()  {
                        match ch.as_rule() {
                            Rule::wall => l.push(Obj::Wall),
                            Rule::container => l.push(Obj::Box),
                            Rule::bot => {
                                start = (r, c);
                                l.push(Obj::Bot);
                            }
                            Rule::empty => l.push(Obj::Empty),
                            _ => unreachable!()
                        }
                    }
                    map.push(l);
                }
            }
            Rule::moves => {
                for line in top.into_inner() {
                    moves.append(&mut line.into_inner().map(|c| {
                        match c.as_rule() {
                            Rule::left => Dir::Left,
                            Rule::right => Dir::Right,
                            Rule::down => Dir::Down,
                            Rule::up => Dir::Up,
                            _ => unreachable!()
                        }
                    }).collect::<Vec<_>>());
                }
            }
            Rule::EOI => {}
            _ => unreachable!()
        }
    }
    let h = map.len();
    let w = map[0].len();
    let map = Array::from_shape_vec((h, w), map.into_iter().flatten().collect()).unwrap();
    (map, moves, start)
}

fn part1(map : &Array2<Obj>, moves : &Vec<Dir>, start: &(usize, usize)) -> i32 {
    let (mut r, mut c) = start;
    let (h, w) = map.dim();
    let mut map = map.clone();
    println!("{:#?}", map);
    println!("");
    //for m in moves.iter().take(6).collect::<Vec<_>>() {
    for m in moves {
        let mut sl = match m {
            Dir::Right  => map.slice_mut(s![r, c..w]),
            Dir::Left => map.slice_mut(s![r, 0..=c;-1]),
            Dir::Up    => map.slice_mut(s![0..=r;-1, c]),
            Dir::Down  => map.slice_mut(s![r..h, c]),
        };
        println!("{:#?}", sl);
        let mv = if sl[1] == Obj::Empty {
            sl[1] = Obj::Bot;
            sl[0] = Obj::Empty;
            true
        } else if sl[1] == Obj::Wall {
            false
        } else {
            let mut idx = 2;
            while sl[idx] != Obj::Wall && sl[idx] != Obj::Empty {
                idx += 1;
            }
            if sl[idx] == Obj::Wall {
                false
            } else {
                for i in (1..=idx).rev() {
                    sl[i] = sl[i - 1].clone();
                }
                sl[0] = Obj::Empty;
                true
            }
        };
        if mv {
            match m {
                Dir::Left  => c -= 1,
                Dir::Right => c += 1,
                Dir::Up    => r -= 1,
                Dir::Down  => r += 1
            }
        }
    };
    let mut score = 0;
    map.indexed_iter().for_each(|((r, c), p)| {
        if *p == Obj::Box {
            score += r * 100 + c;
        }
    });
    println!("{:#?}", map);
    println!("{:#?}", score);
    0
}

pub fn day15() -> (i32, i32) {
    let (map, moves, start) = parse_input();
    (part1(&map, &moves, &start), 0)
}

