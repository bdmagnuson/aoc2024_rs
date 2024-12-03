use std::fs;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "day02.pest"]
struct Day02Parser;

fn is_safe1 (v : &[i32]) -> bool {
    let deltas : Vec<i32> = v.iter().zip(v[1..].iter()).map(|(l, r)| l - r).collect();
    let alldecr = deltas.iter().all(|x| *x > 0);
    let allincr = deltas.iter().all(|x| *x < 0);
    let anydecr = deltas.iter().any(|x| *x > 0);
    let anyincr = deltas.iter().any(|x| *x < 0);
    let inrange = deltas.iter().all(|x| (x.abs() >= 1) && (x.abs() <= 3));
    (alldecr || allincr) && !(anydecr && anyincr) && inrange
}

fn is_safe2 (v : &[i32]) -> bool {
    let vs : Vec<Vec<i32>> = (0..v.len()).map(|x| { let mut _v = v.to_owned(); _v.remove(x); _v }).collect();
    vs.iter().any(|v| is_safe1(v))
}

pub fn day02() -> (i32, i32) {
    let data = fs::read_to_string("input/day02.txt").expect("Unable to read file");
    let file = Day02Parser::parse(Rule::file, &data)
             .expect("parse failed")
             .next().unwrap();

    let reports : Vec<Vec<i32>> =
        file.into_inner().filter_map(|record| {
            match record.as_rule() {
                Rule::record => {
                    Some(record.into_inner().map(|f| f.as_str().parse::<i32>().unwrap()).collect())
                }
                Rule::EOI => { None }
                _ => unreachable!()
            }
        }).collect();

    let part1 : i32 = reports.iter().map(|v| if is_safe1(v) {1} else {0}).sum();
    let part2 : i32 = reports.iter().map(|v| if is_safe2(v) {1} else {0}).sum();
    (part1, part2)
}
