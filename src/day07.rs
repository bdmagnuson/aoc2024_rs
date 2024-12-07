use std::fs; 
use pest::Parser;
use pest_derive::Parser;
use itertools::Itertools;


#[derive(Parser)]
#[grammar = "day07.pest"]
struct Day07Parser;

fn parse_input() -> Vec<(u64, Vec<u64>)> {
    let data = fs::read_to_string("input/day07.txt").expect("Unable to read file");
    let file = Day07Parser::parse(Rule::file, &data)
             .expect("parse failed")
             .next().unwrap();

    let mut lines = Vec::new();
    for record in file.into_inner() {
        match record.as_rule() {
            Rule::line => {
                let ops = record.into_inner().map(|o| o.as_str().parse::<u64>().unwrap()).collect::<Vec<_>>();
                lines.push((ops[0], ops[1..].to_vec()));
            }
            Rule::EOI => {}
            _ => unreachable!()
        }
    }
    lines
}

#[derive(Clone,Copy,Eq,PartialEq,Debug)]
enum Op {
    Mult,
    Add,
    Concat
}

fn combs(sz: usize, concat: bool) -> Vec<Vec<Op>> {
    if concat {
        [Op::Mult, Op::Add, Op::Concat].into_iter().combinations_with_replacement(sz).collect::<Vec<_>>()
    } else {
        [Op::Mult, Op::Add].into_iter().combinations_with_replacement(sz).collect::<Vec<_>>()
    }
}

fn alu(ops: &[Op], input: &[u64]) -> u64 {
    let mut acc = input[0];
    for (op, v) in ops.iter().zip(input[1..].iter()) {
        match op {
            Op::Add => acc += v,
            Op::Mult => acc *= v,
            Op::Concat => acc = (acc * (10_u64.pow(v.ilog10() + 1))) + v
        }
    }
    acc
}

fn part1(input: &Vec<(u64, Vec<u64>)>) -> u64 {
    let mut res = 0;
    for (r, ops) in input {
        if combs(ops.len() - 1, false).iter().any(|o| alu(o, ops) == *r) {
            res += r;
        }
    }
    res
}

fn part2(input: &Vec<(u64, Vec<u64>)>) -> u64 {
    let mut res = 0;
    for (r, ops) in input {
        if combs(ops.len() - 1, true).iter().any(|o| alu(o, ops) == *r) {
            res += r;
        }
    }
    res
}

pub fn day07() -> (u64, u64) {
    let input = parse_input();
    (part1(&input), part2(&input))
}
