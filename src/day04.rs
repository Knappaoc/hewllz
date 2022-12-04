use std::{io::Read, ops::RangeInclusive};

use anyhow::{Context, Result};

pub fn main(input: impl Read) -> Result<String> {
    main_02(input)
}

fn main_01(input: impl Read) -> Result<String> {
    let lines = crate::utils::get_lines(input);
    let mut count = 0;
    for line in lines {
        let line = line?;
        let (a, b) = line.split_once(',').context("missing comma delimiter")?;
        let ra = get_range(a)?;
        let rb = get_range(b)?;
        if full_overlap(&ra, &rb) {
            count += 1;
        }
    }
    Ok(count.to_string())
}

fn full_overlap(ra: &RangeInclusive<usize>, rb: &RangeInclusive<usize>) -> bool {
    contains(&rb, &ra) || contains(&ra, &rb)
}

fn contains(ra: &RangeInclusive<usize>, rb: &RangeInclusive<usize>) -> bool {
    ra.contains(&rb.start()) && ra.contains(&rb.end())
}

fn any_overlap(ra: &RangeInclusive<usize>, rb: &RangeInclusive<usize>) -> bool {
    full_overlap(&rb, &ra) || cross(&ra, &rb)
}

fn cross(ra: &RangeInclusive<usize>, rb: &RangeInclusive<usize>) -> bool {
    ra.contains(&rb.start()) || ra.contains(&rb.end())
}

fn get_range(s: &str) -> Result<RangeInclusive<usize>> {
    let (f, t) = s.split_once('-').context("missing hyphen delimiter")?;
    let f = f.parse()?;
    let t = t.parse()?;
    Ok(f..=t)
}

fn main_02(input: impl Read) -> Result<String> {
    let lines = crate::utils::get_lines(input);
    let mut count = 0;
    for line in lines {
        let line = line?;
        let (a, b) = line.split_once(',').context("missing comma delimiter")?;
        let ra = get_range(a)?;
        let rb = get_range(b)?;
        if any_overlap(&ra, &rb) {
            count += 1;
        }
    }
    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::{main_01, main_02};

    #[test]
    fn test_main_01() {
        let file = indoc!(
            "2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8"
        );
        let res = main_01(file.as_bytes()).unwrap();
        assert_eq!(&*res, "2");
    }
    #[test]
    fn test_main_02() {
        let file = indoc!(
            "2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8"
        );
        let res = main_02(file.as_bytes()).unwrap();
        assert_eq!(&*res, "4");
    }
}
