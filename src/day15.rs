use std::fs; 
use pest::Parser;
use pest_derive::Parser;
use ndarray::{Array, Array2, s};
use rustc_hash::FxHashSet as HashSet;

#[derive(Parser)]
#[grammar = "day15.pest"]
struct Day15Parser;

type Pt = (usize, usize);

#[derive(Debug,PartialEq,Eq,Clone)]
enum Obj {
    Wall,
    LBox,
    RBox,
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

fn parse_input(part2: bool) -> (Array2<Obj>, Vec<Dir>, (usize, usize)) {
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
                        let obj = match ch.as_rule() {
                            Rule::wall => Obj::Wall,
                            Rule::container => Obj::Box,
                            Rule::bot => {
                                start = (r, c * 2);
                                Obj::Bot
                            }
                            Rule::empty => Obj::Empty,
                            _ => unreachable!()
                        };
                        if part2 {
                            match obj {
                                Obj::Wall  => { l.push(Obj::Wall);  l.push(Obj::Wall);  }
                                Obj::Box   => { l.push(Obj::LBox);  l.push(Obj::RBox);  }
                                Obj::Bot   => { l.push(Obj::Bot);   l.push(Obj::Empty); }
                                Obj::Empty => { l.push(Obj::Empty); l.push(Obj::Empty); }
                                _ => unreachable!()
                            }
                        } else {
                            l.push(obj);
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
    //for m in moves.iter().take(6).collect::<Vec<_>>() {
    for m in moves {
        let mut sl = match m {
            Dir::Right  => map.slice_mut(s![r, c..w]),
            Dir::Left => map.slice_mut(s![r, 0..=c;-1]),
            Dir::Up    => map.slice_mut(s![0..=r;-1, c]),
            Dir::Down  => map.slice_mut(s![r..h, c]),
        };
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
    score as i32
}

fn up_cells(map: &Array2<Obj>, r: usize, c: usize) -> Option<HashSet<Pt>> {
    let mut set = HashSet::default();
    match map[[r,c]] {
        Obj::Bot => {
            set.insert((r,c));
            set.extend(up_cells(map, r - 1, c)?);
            Some(set)
        },
        Obj::Empty => {
            Some(set)
        }
        Obj::Wall => None,
        Obj::LBox => {
            set.insert((r,c));
            set.insert((r,c+1));
            set.extend(up_cells(map, r - 1, c)?);
            set.extend(up_cells(map, r - 1, c + 1)?);
            Some(set)
        }
        Obj::RBox => {
            set.insert((r,c));
            set.insert((r,c-1));
            set.extend(up_cells(map, r - 1, c)?);
            set.extend(up_cells(map, r - 1, c - 1)?);
            Some(set)
        }
        _ => unreachable!()
    }

}

fn down_cells(map: &Array2<Obj>, r: usize, c: usize) -> Option<HashSet<Pt>> {
    let mut set = HashSet::default();
    match map[[r,c]] {
        Obj::Bot => {
            set.insert((r,c));
            set.extend(down_cells(map, r + 1, c)?);
            Some(set)
        },
        Obj::Empty => Some(set),
        Obj::Wall => None,
        Obj::LBox => {
            set.insert((r,c));
            set.insert((r,c+1));
            set.extend(down_cells(map, r + 1, c)?);
            set.extend(down_cells(map, r + 1, c + 1)?);
            Some(set)
        }
        Obj::RBox => {
            set.insert((r,c));
            set.insert((r,c-1));
            set.extend(down_cells(map, r + 1, c)?);
            set.extend(down_cells(map, r + 1, c - 1)?);
            Some(set)
        }
        _ => unreachable!()
    }

}

fn part2(map : &Array2<Obj>, moves : &Vec<Dir>, start: &(usize, usize)) -> i32 {
    let (mut r, mut c) = start;
    let (_, w) = map.dim();
    let mut map = map.clone();
    for m in moves {
        match m {
            Dir::Left => {
                let mut sl = map.slice_mut(s![r, 0..=c;-1]);
                let mut idx = 1;
                let mut mv = true;
                loop {
                    match sl[idx] {
                        Obj::Wall  => { mv = false; break; }
                        Obj::Empty => { break; }
                        _          => { idx += 1; }
                    }
                }
                if mv {
                    for i in (1..=idx).rev() {
                        sl[i] = sl[i - 1].clone();
                    }
                    sl[0] = Obj::Empty;
                    c -= 1;
                }
            }
            Dir::Right => {
                let mut sl = map.slice_mut(s![r, c..w]);
                let mut idx = 1;
                let mut mv = true;
                loop {
                    match sl[idx] {
                        Obj::Wall  => { mv = false; break; }
                        Obj::Empty => { break; }
                        _          => { idx += 1; }
                    }
                }
                if mv {
                    for i in (1..=idx).rev() {
                        sl[i] = sl[i - 1].clone();
                    }
                    sl[0] = Obj::Empty;
                    c += 1;
                }
            }
            Dir::Up => {
                match up_cells(&map, r, c) {
                    None => {},
                    Some(s) => {
                        let l = s.iter().collect::<Vec<_>>();
                        let mut mc = map.clone();
                        for (r, c) in &l {
                            mc[[*r,*c]] = Obj::Empty;
                        }
                        for (r, c) in l {
                            mc[[r-1,*c]] = map[[*r,*c]].clone();
                        }
                        map = mc;
                        r -= 1;
                    }
                }
            },
            Dir::Down => {
                match down_cells(&map, r, c) {
                    None => {},
                    Some(s) => {
                        let l = s.iter().collect::<Vec<_>>();
                        let mut mc = map.clone();
                        for (r, c) in &l {
                            mc[[*r,*c]] = Obj::Empty;
                        }
                        for (r, c) in l {
                            mc[[r+1,*c]] = map[[*r,*c]].clone();
                        }
                        map = mc;
                        r += 1;
                    }
                }
            },
        }
    }
    let mut score = 0;
    map.indexed_iter().for_each(|((r, c), p)| {
        if *p == Obj::LBox {
            score += r * 100 + c;
        }
    });
    score as i32
}

pub fn day15() -> (i32, i32) {
    let (map, moves, start) = parse_input(false);
    let p1 = part1(&map, &moves, &start);
    let (map, moves, start) = parse_input(true);
    let p2 = part2(&map, &moves, &start);
    (p1, p2)
}

