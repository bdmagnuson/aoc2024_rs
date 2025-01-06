use std::fs; 
use pest::Parser;
use pest_derive::Parser;
use rustc_hash::FxHashMap as HashMap;
use itertools::Itertools;
use rand::Rng;

#[derive(Parser)]
#[grammar = "day24.pest"]
struct Day24Parser;

#[derive(Debug,Clone,Eq,PartialEq,Hash)]
enum Op {
    Xor,
    Or,
    And
}

#[derive(Debug,Clone,Eq,PartialEq,Hash)]
enum Node {
    Const(u64),
    Gate { in0: String, in1: String, op: Op}
}

fn eval(m: &mut HashMap<String, Node>, n: String, depth: u32) -> Option<u64> {
    if depth > 10 {
        None
    } else {
        let l = m.get(&n).unwrap().clone();
        let next = match l {
            Node::Const(v) => Some(v),
            Node::Gate {in0, in1, op, ..} => {
                let v0 = eval(m, in0.to_string(), depth + 1)?;
                let v1 = eval(m, in1.to_string(), depth + 1)?;
                match op {
                    Op::And => Some(v0 & v1),
                    Op::Or => Some(v0 | v1),
                    Op::Xor => Some(v0 ^ v1)
                }
            }
        };
        m.insert(n, Node::Const(next.unwrap()));
     next
    }
}

fn result(m: &HashMap<String,Node>) -> Option<u64> {
    let mut res = 0;
    let mut zs = m.keys().filter(|k| k.starts_with('z')).collect::<Vec<_>>();
    zs.sort();
    let mut mm = m.clone();
    for (s, z) in zs.iter().enumerate() {
        res |= eval(&mut mm, z.to_string(), 0)? << s;
    }
    Some(res)
}

fn _test(m: &HashMap<String, Node>) -> HashMap<usize, Vec<usize>> {
    let width = m.keys().filter(|k| k.starts_with('z')).collect::<Vec<_>>().len();
    let mut bad_bits = HashMap::default();
    for ib in 0..width-1 {
        let x : u64 = 1 << ib;
        let y : u64 = 0;
        let z = x + y;
        let mut mm : HashMap<String,Node> = m.clone();
        for b in 0..width {
            mm.insert(format!("x{:02}", b),Node::Const((x >> b) & 1));
            mm.insert(format!("y{:02}", b),Node::Const((y >> b) & 1));
        }
        let z_test = result(&mm).unwrap();
        let z_bad = z ^ z_test;
        for ob in 0..width {
            if (z_bad >> ob) & 0x1 == 1 {
                bad_bits.entry(ib).and_modify(|s: &mut Vec<usize>| s.push(ob)).or_insert(vec![ob]);
            }
        }
    }
    /*
    for ib in 0..width-1 {
        let x : u64 = 1 << ib;
        let y : u64 = 1 << ib;
        let z = x + y;
        let mut mm : HashMap<String,Node> = m.clone();
        for b in 0..width {
            mm.insert(format!("x{:02}", b),Node::Const((x >> b) & 1));
            mm.insert(format!("y{:02}", b),Node::Const((y >> b) & 1));
        }
        let z_test = result(&mm).unwrap();
        let z_bad = z ^ z_test;
        for ob in 0..width {
            if (z_bad >> ob) & 0x1 == 1 {
                bad_bits.entry(ib).and_modify(|s: &mut Vec<usize>| s.push(ob)).or_insert(vec![ob]);
            }
        }
    }
    */
    bad_bits
}

fn test2(m: &HashMap<String, Node>) -> bool {
    let width = m.keys().filter(|k| k.starts_with('z')).collect::<Vec<_>>().len();
    let mut rng = rand::thread_rng();
    for _ in 0..width-1 {
        let mut m : HashMap<String,Node> = m.clone();
        let x : u64 = rng.gen_range(0..(1 << (width - 1)));
        let y : u64 = rng.gen_range(0..(1 << (width - 1)));
        let z = x + y;
        for b in 0..width-1 {
            m.insert(format!("x{:02}", b),Node::Const((x >> b) & 1));
            m.insert(format!("y{:02}", b),Node::Const((y >> b) & 1));
        }
        match result(&m) {
            None => { 
               //println!("none");
               return false;
            }
            Some(z_test) => {
                if z ^ z_test != 0 {
                    //println!("{} + {} = {} (expected {})", x, y, z_test, z);
                    return false;
                }
            }
        }
    }
    true
}

// Find all outputs that use this input
fn _fanout(m: &HashMap<String, Node>, input: &str) -> Vec<String>  {
    let mut gates = Vec::default();
    for (output, gate) in m.iter() {
        match gate {
            Node::Const(_) => {},
            Node::Gate { in0, in1, ..} => {
                if in0 == input || in1 == input {
                    gates.push(output.to_string());
                }
            }
        }
    }
    gates
}

// Find all gates that connect in to outs with a DFS
fn _walkback(m: &HashMap<String,Node>, i: usize, outs: &[usize]) -> Vec<String> {
    let i = format!("x{:02}", i);
    let outs = outs.iter().map(|o| format!("z{:02}", o)).collect::<Vec<String>>();
    let mut paths = Vec::default();


    let mut stack: Vec<Vec<String>> = vec![vec![i.clone()]];
    while let Some(g) = stack.pop() {
        for o in _fanout(m, &g[g.len() - 1]) {
            let mut gp = g.clone();
            if outs.contains(&o) {
                gp.push(o);
                paths.push(gp);
            } else if o.starts_with('z') {
                continue;
            } else {
                gp.push(o);
                stack.push(gp);
            }
        }
    }
    let ret = paths.into_iter().flatten().collect::<Vec<_>>();
    println!("Paths from {} to {:?} = {:?}", i, outs, ret);
    ret
}

/*
fn pairs<'a, T>(vals: &'a Vec<T>) -> Box<dyn Iterator<Item = Vec<(&T, &T)>> + 'a>
where T:Clone+PartialEq+Eq
{
    let l = vals.len();
    if l % 2 != 0 {
        panic!("Iterator must be even");
    }
    let first = vec![(&vals[0], &vals[1])];
    if l == 2 {
        Box::new(vec![first].into_iter())
    } else {
        Box::new(vals.iter().combinations(2).map(|c| {
            let rest : Vec<&T> = vals.iter().filter(|item| !c.contains(item)).collect();
            iter::repeat(first).zip(pairs(&rest)).map(|(l, r)| l.extend(r))
        }))
    }
}
*/

fn part2(m: &HashMap<String, Node>) -> String {
    /*
    let errors = test(m);
    let mut outputs : Vec<String> = errors
        .iter()
        .flat_map(|(i, os)| walkback(m, *i, os)).collect();
    outputs.sort();
    outputs.dedup();
    outputs.retain(|o| !o.starts_with('x'));
    outputs.sort();
    println!("{:?} {}", outputs, outputs.len());
    return "".to_string();
    */

    let outputs =
        ["cph".to_string(),
         "z19".to_string(),
         "hgj".to_string(),
         "z33".to_string(),
         "npf".to_string(),
         "z13".to_string(),
         "gws".to_string(),
         "nnt".to_string()];

        
    for swaps in outputs.iter().permutations(8) {
        let mut new_map = m.clone();
        for s in swaps.iter().tuples::<(&&String, &&String)>() {
            let o0 = m.get(*s.0).unwrap();
            let o1 = m.get(*s.1).unwrap();
            new_map.insert(s.0.to_string(), o1.clone());
            new_map.insert(s.1.to_string(), o0.clone());
        }
        if test2(&new_map) {
            return {let mut s = swaps.clone(); s.sort(); s}.iter().join(",").to_string();
        }
    }
    "".to_string()
}

fn parse_input() -> HashMap<String,Node> {
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
    map
}

pub fn day24() -> (u64, String) {
    let map = parse_input();
    let part1 = result(&map).unwrap();
    (part1, part2(&map))
}




