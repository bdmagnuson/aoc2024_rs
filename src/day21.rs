use std::fs; 
use rustc_hash::FxHashMap as HashMap;
use std::cmp::Ordering::{Less,Greater};

fn parse_input() -> Vec::<String> {
    fs::read_to_string("input/day21.txt")
        .expect("Diable to read file")
        .lines().map(|s| s.to_string()).collect::<Vec<_>>()
}

fn dir2num (code: &str) -> String {
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
                    (Less, Greater) => false,
                    (Greater, Greater) => false,
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
   input
}


fn dir2dir (code: &str) -> String {
   let positions: HashMap<char, (usize, usize)> = HashMap::from_iter([
       (' ', (0, 1)), ('^', (1, 1)), ('A', (2, 1)),
       ('<', (0, 0)), ('v', (1, 0)), ('>', (2, 0))
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

        let avoid_blank = *cur == (0,0) || *dest == (0,0);
        let lr_first = avoid_blank ||
            match (dest.1.cmp(&cur.1), dest.0.cmp(&cur.0)) {
                (Greater, Less) => true,
                (Less, Less) => true,
                (Less, Greater) => false,
                (Greater, Greater) => false,
                _ => false
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
    input
}

fn complexity1(input: &str) -> usize {
    let seq = dir2dir(&dir2dir(&dir2num(input)));
    let num = {
        let mut chars = input.chars();
        chars.next_back();
        chars.as_str()
    }.parse::<usize>().unwrap();
    seq.len() * num
}

pub fn day21() -> (usize, usize) {
    let part1 = parse_input().iter().map(|s| complexity1(s)).sum();
//    let part2 = parse_input().iter().map(|s| complexity2(s)).sum();
    (part1,0)
}

