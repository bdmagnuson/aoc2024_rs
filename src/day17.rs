use std::fs;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "day17.pest"]
struct Day17Parser;

#[derive(Debug,Clone)]
struct Cpu {
    reg_a : u64,
    reg_b : u64,
    reg_c : u64,
    pc    : usize,
    prog  : Vec<u64>,
    output : Vec<u64>,
    halted : bool
}

fn parse_input() -> Cpu {
    let data = fs::read_to_string("input/day17.txt").expect("Diable to read file");
    let file = Day17Parser::parse(Rule::file, &data)
             .expect("parse failed")
             .next().unwrap();

    let mut reg_a  = 0;
    let mut reg_b = 0;
    let mut reg_c = 0;
    let pc = 0;
    let mut prog = Vec::default();
    for elem in file.into_inner() {
        match elem.as_rule() {
            Rule::ra => { reg_a = elem.into_inner().next().unwrap().as_str().parse::<u64>().unwrap(); }
            Rule::rb => { reg_b = elem.into_inner().next().unwrap().as_str().parse::<u64>().unwrap(); }
            Rule::rc => { reg_c = elem.into_inner().next().unwrap().as_str().parse::<u64>().unwrap(); }
            Rule::prog => { prog = elem.into_inner().map(|n| n.as_str().parse::<u64>().unwrap()).collect::<Vec<_>>();}
            Rule::EOI => {}
            _ => unreachable!()
        }
    }
    Cpu { reg_a, reg_b, reg_c, pc, prog, halted: false, output: Vec::default()}
}

fn exec(cpu: &mut Cpu) {
    let lit = cpu.prog[cpu.pc + 1] as u64;
    let combo =
        match cpu.prog[cpu.pc + 1] {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => cpu.reg_a,
            5 => cpu.reg_b,
            6 => cpu.reg_c,
            _ => unreachable!()
        };
    let mut branch_taken = false;
    match cpu.prog[cpu.pc] {
        0 => { cpu.reg_a = cpu.reg_a / (1 << combo); }
        1 => { cpu.reg_b = cpu.reg_b ^ lit; }
        2 => { cpu.reg_b = combo % 8; }
        3 => { 
            if cpu.reg_a != 0 {
                branch_taken = true;
                cpu.pc = lit as usize;
            }
        }
        4 => { cpu.reg_b = cpu.reg_b ^ cpu.reg_c; }
        5 => { cpu.output.push(combo % 8); }
        6 => { cpu.reg_b = cpu.reg_a / (1 << combo); }
        7 => { cpu.reg_c = cpu.reg_a / (1 << combo); }
        _ => unreachable!()
    }
    if !branch_taken {
        cpu.pc += 2;
    }
    if cpu.pc >= cpu.prog.len() {
        cpu.halted = true;
    }
}

fn part1(cpu: &Cpu) -> String {
    let mut cpu = cpu.clone();
    while !cpu.halted {
        exec(&mut cpu);
    }
    format!("{}", cpu.output.iter().map(|d| d.to_string()).collect::<Vec<_>>().join(","))
}

/*
  Basically run the program backwards after decompling.  reg_a is the only value that matters since
  b and c get overwritten with values rooted in a.  a ends as 0 and is /8 every round so the
  iteration previous it had to be somewhere in the range of a0 = 0..7 the loop before a1 =
  a0*8..(a0+1)*8 and so on.  So we only have to check 8 posibilities per loop to see which one
  creates the expect output.

   0: b = a % 8                  2,4
   2: b = b ^ 5                  1,5                                                                                
   4: c = a / (1 << b)           7,5                                                                                
   6: a = a / (1 << 3)           0,3                                                                                
   8: b = b ^ c                  4,1                                                                                
  10: b = b ^ 6                  1,6                                                                                
  12: out <- b % 8               5,5                                                                                
  14: if a == 0 halt else jmp 0  3,0                                                                                
*/

fn reverse(a: u64, v: &[u64]) -> Option<u64> {
    if v.len() == 0 {
        return Some(a)
    }
    for n in a*8..(a+1)*8 {
        let mut b = n % 8;
        b = b ^ 5;
        let c = n / (1 << b);
        b = b ^ c;
        b = b ^ 6;
        if b % 8 == v[0] {
            let ret = reverse(n, &v[1..]);
            if ret.is_none() {
                continue;
            } else {
                return ret;
            }
        }
    }
    None
}

fn part2(cpu: &Cpu) -> u64 {
    let mut v = cpu.prog.clone();
    v.reverse();
    reverse(0, &v).unwrap()
}

pub fn day17() -> (String, u64) {
    let cpu = parse_input();
    (part1(&cpu),part2(&cpu))
}

