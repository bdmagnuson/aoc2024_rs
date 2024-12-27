use std::fs; 
use rustc_hash::FxHashMap as HashMap;
use std::cmp::Ordering::{Less,Greater};
use itertools::Itertools;
use memoize::memoize;

fn parse_input() -> Vec::<String> {
    fs::read_to_string("input/day21.txt")
        .expect("Diable to read file")
        .lines().map(|s| s.to_string()).collect::<Vec<_>>()
}

#[memoize]
fn cost(from: char, to: char, level: u32) -> u64 {
   let positions: HashMap<char, (usize, usize)> = HashMap::from_iter([
       (' ', (0, 1)), ('^', (1, 1)), ('A', (2, 1)),
       ('<', (0, 0)), ('v', (1, 0)), ('>', (2, 0))
   ]);
   let cur = positions.get(&from).unwrap();
   let dest = positions.get(&to).unwrap();

   let mut lr = String::default();
   let mut ud = String::default();

   ud.push_str(&"^".repeat(dest.1.saturating_sub(cur.1)));
   ud.push_str(&"v".repeat(cur.1.saturating_sub(dest.1)));
   lr.push_str(&">".repeat(dest.0.saturating_sub(cur.0)));
   lr.push_str(&"<".repeat(cur.0.saturating_sub(dest.0)));

   let lr_first = 
       if *cur == (0,0) {
           true
       } else if *dest == (0,0) {
           false
       } else {
           match (dest.1.cmp(&cur.1), dest.0.cmp(&cur.0)) {
               (Greater, Less) => true,
               (Less, Less) => true,
               _ => false
           }
       };

   let mut input = String::default();
   if lr_first {
       input.push_str(&lr);
       input.push_str(&ud);
   } else {
       input.push_str(&ud);
       input.push_str(&lr);
   }
   input.push_str(&"A");

   if level == 0 {
       input.len() as u64
   } else {
       let mut leading_a = "A".to_string();
       leading_a.push_str(&input);
       leading_a.chars().tuple_windows().map(|(l, r)| cost(l, r, level - 1)).sum()
   }
}

fn dir2num (code: &str, level: u32) -> u64 {
    let positions: HashMap<char, (usize, usize)> = HashMap::from_iter([
        ('7', (0, 3)), ('8', (1, 3)), ('9', (2, 3)),
        ('4', (0, 2)), ('5', (1, 2)), ('6', (2, 2)),
        ('1', (0, 1)), ('2', (1, 1)), ('3', (2, 1)),
        (' ', (0, 0)), ('0', (1, 0)), ('A', (2, 0))
    ]);
    let mut input = String::default();
    let mut cur = positions.get(&'A').unwrap();
    for c in code.chars() {
        let dest = positions.get(&c).unwrap();
        let mut lr = String::default();
        let mut ud = String::default();

        ud.push_str(&"^".repeat(dest.1.saturating_sub(cur.1)));
        ud.push_str(&"v".repeat(cur.1.saturating_sub(dest.1)));
        lr.push_str(&">".repeat(dest.0.saturating_sub(cur.0)));
        lr.push_str(&"<".repeat(cur.0.saturating_sub(dest.0)));

        let lr_first =
            if cur.1 == 0 && dest.0 == 0 {
                false
            } else if dest.1 == 0 && cur.0 == 0 {
                true
            } else {
                match (dest.1.cmp(&cur.1), dest.0.cmp(&cur.0)) {
                    (Greater, Less) => true,
                    (Less, Less) => true,
                    _ => false
                }
            };

        if lr_first {
            input.push_str(&lr);
            input.push_str(&ud);
        } else {
            input.push_str(&ud);
            input.push_str(&lr);
        }
        input.push_str(&"A");
        cur = dest;
   }
   if level == 0 {
       input.len() as u64
   } else {
       let mut leading_a = "A".to_string();
       leading_a.push_str(&input);
       leading_a.chars().tuple_windows().map(|(l, r)| cost(l, r, level - 1)).sum()
   }
}

fn complexity(input: &str, level: u32) -> u64 {
    let seq = dir2num(input, level);
    let num = {
        let mut chars = input.chars();
        chars.next_back();
        chars.as_str()
    }.parse::<usize>().unwrap();
    seq * num as u64
}

pub fn day21() -> (u64, u64) {
    let part1 = parse_input().iter().map(|s| complexity(s, 2)).sum();
    let part2 = parse_input().iter().map(|s| complexity(s, 25)).sum();
    (part1,part2)
}

