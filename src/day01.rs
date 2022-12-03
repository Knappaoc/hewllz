use anyhow::Result;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main(input: File) -> Result<()> {
    let lines = BufReader::new(input).lines();
    let lines = lines
        .enumerate()
        .map(|(i, r)| r.expect(&format!("failed to read line {i} of file")));
    let totals = totals(lines);
    //let max = totals.max().expect("expected a nonzero maximum");
    // println!("maximum is {max}");
    let tops = totals.sorted_by(|a, b| b.cmp(a)).take(3).sum::<u32>();
    println!("total of top 3 is {tops}");
    Ok(())
}

fn totals(lines: impl Iterator<Item = String>) -> impl Iterator<Item = u32> {
    lines.batching(|it| {
        let mut sum = 0;
        let mut count = 0;
        while let Some(val) = it.next() {
            if val.len() == 0 {
                return Some(sum);
            } else {
                count += 1;
                sum += val
                    .parse::<u32>()
                    .expect(&format!("could not parse {val} as int"));
            }
        }
        if count > 0 {
            Some(sum)
        } else {
            None
        }
    })
}
