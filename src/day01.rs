use anyhow::Result;
use itertools::Itertools;
use std::io::{BufRead, BufReader, Read};

pub fn main(input: impl Read) -> Result<String> {
    main_02(input)
}
// get the sum of the three largest totals carried
fn main_02(input: impl Read) -> Result<String> {
    let lines = BufReader::new(input).lines();
    let lines = lines
        .enumerate()
        .map(|(i, r)| r.expect(&format!("failed to read line {i} of file")));
    let totals = totals(lines);
    //let max = totals.max().expect("expected a nonzero maximum");
    // println!("maximum is {max}");
    let tops = totals.sorted_by(|a, b| b.cmp(a)).take(3).sum::<u32>();
    println!("total of top 3 is {tops}");
    Ok(tops.to_string())
}
// get the largest total carried among all totals
fn main_01(input: impl Read) -> Result<String> {
    let lines = BufReader::new(input).lines();
    let lines = lines
        .enumerate()
        .map(|(i, r)| r.expect(&format!("failed to read line {i} of file")));
    let totals = totals(lines);
    let max = totals.max().expect("expected a nonzero maximum");
    println!("maximum is {max}");
    Ok(max.to_string())
}

/// a bit magic: operating on batches of lines,
/// where each batch is a run with no empty lines between,
/// gets the sum of each run
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

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day01::{main_01, main_02};

    #[test]
    fn test_main_01() {
        let file = indoc!(
            "1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000"
        );
        let res = main_01(file.as_bytes()).unwrap();
        assert_eq!(&*res, "24000");
    }
    #[test]
    fn test_main_02() {
        let file = indoc!(
            "1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000"
        );
        let res = main_02(file.as_bytes()).unwrap();
        assert_eq!(&*res, "45000");
    }
}
