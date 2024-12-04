use std::fs; 
use itertools::iterate;
use std::collections::{HashSet, HashMap};

type XmasMap = HashMap<char, HashSet<(i32,i32)>>;

fn paths(p: (i32, i32)) -> Vec<Vec<(i32, i32)>> {
    let dirs = [
        ( 0,  1),
        ( 0, -1),
        ( 1,  0),
        (-1,  0),
        ( 1,  1),
        ( 1, -1),
        (-1,  1),
        (-1, -1)
    ];
    let mut paths = Vec::new();
    for (r, c) in dirs {
        paths.push(iterate(p, |&(pr, pc)| (pr + r, pc + c) ).take(4).collect());
    }
    paths
}

fn paths2(p: (i32, i32)) -> Vec<Vec<(i32, i32)>> {
    let dirs = [
        ( 1,  1),
        ( 1, -1),
        (-1,  1),
        (-1, -1)
    ];
    let mut paths = Vec::new();
    for (r, c) in dirs {
        paths.push(iterate(p, |&(pr, pc)| (pr + r, pc + c) ).take(3).collect());
    }
    paths
}

fn is_xmas (m: &XmasMap, ps: &[(i32, i32)]) -> bool {
    m.get(&'X').expect("No X?").contains(&ps[0]) &&
    m.get(&'M').expect("No M?").contains(&ps[1]) &&
    m.get(&'A').expect("No A?").contains(&ps[2]) &&
    m.get(&'S').expect("No S?").contains(&ps[3])
}

fn is_mas (m: &XmasMap, ps: &[(i32, i32)]) -> bool {
    m.get(&'M').expect("No M?").contains(&ps[0]) &&
    m.get(&'A').expect("No A?").contains(&ps[1]) &&
    m.get(&'S').expect("No S?").contains(&ps[2])

}

fn get_input() -> XmasMap {
    let mut map = HashMap::new();

    for (r, l) in fs::read_to_string("input/day04.txt").expect("Unable to read file").lines().enumerate() {
        for (c, ch) in l.chars().enumerate() {
            let pos = (r as i32, c as i32);
            map.entry(ch).and_modify(|s: &mut HashSet<(i32, i32)>| {s.insert(pos);} ).or_insert(HashSet::from([pos]));
        }
    }
    map
}

fn part1(map: &XmasMap) -> i32 {
    let mut part1 = 0;
    for x in map.get(&'X').expect("no X?").iter() {
        part1 += paths(*x).iter().map(|p| if is_xmas(map, p) {1} else {0}).sum::<i32>();
    }
    part1
}

fn part2(map: &XmasMap) -> i32 {
    let crosses : Vec<(i32, i32)> = map.get(&'M').expect("no M?").iter().flat_map(|x| {
        paths2(*x).iter().filter_map(|p| if is_mas(map, p) {Some(p[1])} else {None}).collect::<Vec<_>>()
    }).collect();
    let counts = crosses.iter().fold(HashMap::new(), |mut acc, x| {acc.entry(x).and_modify(|v : &mut i32| {*v += 1;}).or_insert(1); acc});
    let part2 = counts.values().fold(0, |acc, v| if *v == 2 {acc + 1} else {acc});
    part2
}


pub fn day04() -> (i32, i32) {
    let map = get_input();
    (part1(&map), part2(&map))
}
