use std::fs;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "day01.pest"]
struct Day01Parser;

pub fn day01() -> (i32, i32) {
    let data = fs::read_to_string("input/day01.txt").expect("Unable to read file");
    let file = Day01Parser::parse(Rule::file, &data)
             .expect("parse failed")
             .next().unwrap();

    let pairs : Vec<(i32, i32)> =
        file.into_inner().filter_map(|record| {
            match record.as_rule() {
                Rule::record => {
                    let a : Vec<i32> = record.into_inner().map(|f| f.as_str().parse::<i32>().unwrap()).collect();
                    Some((a[0], a[1]))
                }
                Rule::EOI => { None }
                _ => unreachable!()
            }
        }).collect();
    let (mut ls, mut  rs) : (Vec<i32>, Vec<i32>) = pairs.into_iter().unzip();
    ls.sort();
    rs.sort();
    let sum : i32 = ls.clone().into_iter().zip(rs.clone()).map(|(l, r)| (l - r).abs()).sum();

    let mut score = 0;
    for l in ls {
        score += (l as usize) * rs.clone().into_iter().filter(|r| *r == l).count()
    }

    (sum, score as i32)
}
