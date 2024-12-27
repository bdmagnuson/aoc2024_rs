use std::fs; 
use itertools::Itertools;
use itertools::{iterate, repeat_n};
use rustc_hash::FxHashMap as HashMap;

fn get_input() -> Vec<u64> {
    fs::read_to_string("input/day22.txt").expect("Unable to read file").lines().map(|l| l.parse::<u64>().unwrap()).collect::<Vec<_>>()
}

fn evolve(s0: &u64) -> u64 {
    let s1 = ((s0 * 64) ^ s0) % 16777216;
    let s2 = ((s1 / 32) ^ s1) % 16777216;
    let s3 = ((s2 * 2048) ^ s2) % 16777216;
    s3
}

#[derive(Debug)]
struct Seq {
    prices : Vec<u64>,
    diffs : Vec<i64>
}

fn genseq(seed: u64) -> Seq {
    let vs = iterate(seed, evolve).take(2001).collect::<Vec<_>>();
    Seq {
        prices : vs[1..].iter().map(|v| *v ).collect::<Vec<_>>(),
        diffs : vs.iter().zip(vs[1..].iter()).map(|(a, b)| (*b as i64) % 10 - (*a as i64) % 10).collect::<Vec<_>>()
    }
}

fn part1(vs: &Vec<u64>) -> u64 {
    let mut sum = 0;
    for v in vs.iter() {
        let s = genseq(*v);
        sum += s.prices[s.prices.len() - 1];
    }
    sum
}

fn part2(vs: &Vec<u64>) -> u64 {
    let seqs = vs.iter().map(|v| {
        let mut s = genseq(*v);
        s.prices.iter_mut().for_each(|s| *s %= 10);
        s
    }).collect::<Vec<Seq>>();
    let changes = repeat_n(-2 as i64..=2, 4).multi_cartesian_product().filter(|s| {
        let mut sum = 0;
        for ss in s {
            sum += ss;
            if sum >= 10 || sum <= -10 {
                return false;
            }
        }
        true
    }).collect::<Vec<_>>();
    let mut maps = Vec::new();
    for s in seqs.iter() {
        let mut m = HashMap::default();
        for (idx, d) in s.diffs.windows(4).enumerate() {
            m.entry(d).or_insert(s.prices[idx + 3]);
        }
        maps.push(m);
    }
    let mut max_sum = 0;
    for c in changes  {
        let mut sum = 0;
        for m in maps.iter() {
            sum += m.get(&c[..]).unwrap_or(&0);
        }
        if sum > max_sum {
            max_sum = sum;
        }
    }
    max_sum
}


pub fn day22() -> (u64, u64) {
    let vs = get_input();
    (part1(&vs),part2(&vs))
}

