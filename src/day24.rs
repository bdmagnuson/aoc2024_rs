use std::fs; 
use pest::Parser;
use pest_derive::Parser;
use rustc_hash::FxHashMap as HashMap;
use rand::Rng;

#[derive(Parser)]
#[grammar = "day24.pest"]
struct Day24Parser;

#[derive(Debug,Clone)]
enum Op {
    Xor,
    Or,
    And
}

#[derive(Debug,Clone)]
enum Node {
    Const(u64),
    Gate { in0: String, in1: String, op: Op}
}

fn eval(m: &HashMap<String, Node>, n: String) -> u64 {
    match m.get(&n).unwrap() {
        Node::Const(v) => *v,
        Node::Gate {in0, in1, op} => {
            let v0 = eval(m, in0.to_string());
            let v1 = eval(m, in1.to_string());
            match op {
                Op::And => v0 & v1,
                Op::Or => v0 | v1,
                Op::Xor => v0 ^ v1
            }
        }
    }
}

fn result(m: &HashMap<String,Node>, width: u32) -> u64 {
    let mut res = 0;
    let mut zs = m.keys().filter(|k| k.chars().next().unwrap() == 'z').collect::<Vec<_>>();
    zs.sort();
    for (s, z) in zs.iter().enumerate() {
        res |= eval(&m, z.to_string()) << s;
    }
    res
}

fn test(m: &HashMap<String, Node>, width: u32) {
    let mut m : HashMap<String,Node> = m.clone();
    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        let x : u64 = rng.gen();
        let y : u64 = rng.gen();
        let z : u64 = x + y;
        for b in 0..width {
            m.insert(format!("x{:02}", b),Node::Const((x >> b) & 1));
            m.insert(format!("y{:02}", b),Node::Const((y >> b) & 1));
        }
    }

}

fn parse_input() -> (u64, u64) {
    let data = fs::read_to_string("input/day24.txt").expect("Diable to read file");
    let top = Day24Parser::parse(Rule::top, &data)
             .expect("parse failed")
             .next().unwrap();


    let mut map = HashMap::default();
    for elem in top.into_inner() {
        match elem.as_rule() {
            Rule::constant => {
                let pair = elem.into_inner().map(|r| r.as_str()).collect::<Vec<_>>();
                map.insert(pair[0].to_string(), Node::Const(pair[1].parse::<u64>().unwrap()));
            }
            Rule::eq => {
                let mut operands = Vec::default();
                let mut op = Op::Or;
                for p in elem.into_inner() {
                    match p.as_rule() {
                        Rule::node => {
                            operands.push(p.as_str());
                        }
                        Rule::op => {
                            match p.into_inner().next().unwrap().as_rule() {
                                Rule::and => {
                                    op = Op::And;
                                }
                                Rule::or => {
                                    op = Op::Or;
                                }
                                Rule::xor => {
                                    op = Op::Xor;
                                }
                                _ => unreachable!()
                            }
                        }
                        _ => unreachable!()

                    }
                }
                map.insert(operands[2].to_string(), Node::Gate {in0: operands[0].to_string(), in1: operands[1].to_string(), op});
            }
            Rule::EOI => {}
            _ => unreachable!()
        }
    }
    let mut zs = map.keys().filter(|k| k.chars().next().unwrap() == 'z').collect::<Vec<_>>();
    zs.sort();

    let mut part1 = 0;
    for (s, z) in zs.iter().enumerate() {
        part1 |= eval(&map, z.to_string()) << s;
    }

    (part1,0)
}

pub fn day24() -> (u64, u64) {
    parse_input()
}

