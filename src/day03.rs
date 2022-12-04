use anyhow::{anyhow, Context, Result};
use linq::iter::Enumerable;
use std::{
    collections::HashSet,
    io::{BufRead, BufReader, Read},
};
use tap::{Conv, TryConv};

use crate::utils::get_lines;

pub fn main(input: impl Read) -> Result<String> {
    main_02(input)
}
fn main_01(input: impl Read) -> Result<String> {
    let mut sum = 0u64;
    for line in get_lines(input) {
        let line = line?;
        let (a, b) = line.split_at(line.len() / 2);
        let common = get_common_item(a, b)?;
        let priority = priority(common)?;
        sum += priority.conv::<u64>();
    }
    Ok(sum.to_string())
}
fn main_02(input: impl Read) -> Result<String> {
    let lines = get_lines(input);
    let groups = lines.array_chunks::<3>();
    let mut sum = 0u64;
    for group in groups {
        let [a, b, c] = group;
        let common = get_common_item_n(&[&a?, &b?, &c?])?;
        sum += priority(common)? as u64;
    }
    Ok(sum.to_string())
}

fn priority(c: char) -> Result<u8> {
    let (letter, pri) = match c {
        'a'..='z' => (b'a', 1),
        'A'..='Z' => (b'A', 27),
        _ => Err(anyhow!("character '{c}' has no known priority"))?,
    };
    Ok(c.try_conv::<u8>().unwrap() - letter + pri)
}
fn as_set(s: &str) -> HashSet<char> {
    let mut set = HashSet::new();
    for c in s.chars() {
        set.insert(c);
    }
    set
}
fn get_common_item_n(ss: &[&str]) -> Result<char> {
    let sets = ss.into_iter().map(|s| as_set(*s));
    let inters: HashSet<char> = sets
        .reduce(|a, b| &a & &b)
        .context("no intersection of 0 sets")?;
    let common = inters
        .into_iter()
        .single()
        .context("no common items, or too many")?;
    Ok(common)
}

fn get_common_item(a: &str, b: &str) -> Result<char> {
    get_common_item_n(&[a, b])
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day03::{get_common_item, main_01, main_02, priority};

    #[test]
    fn test_main_01() {
        let file = indoc!(
            "vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw"
        );
        let res = main_01(file.as_bytes()).unwrap();
        assert_eq!(&*res, "157");
    }
    #[test]
    fn test_main_02() {
        let file = indoc!(
            "vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw"
        );
        let res = main_02(file.as_bytes()).unwrap();
        assert_eq!(&*res, "70");
    }
    #[test]
    fn test_get_priority() {
        assert_eq!(16, priority('p').unwrap());
        assert_eq!(38, priority('L').unwrap());
        assert_eq!(42, priority('P').unwrap());
        assert_eq!(22, priority('v').unwrap());
        assert_eq!(20, priority('t').unwrap());
        assert_eq!(19, priority('s').unwrap());
    }
    #[test]
    fn test_get_common_items() {
        assert_eq!(
            'p',
            get_common_item("vJrwpWtwJgWr", "hcsFMMfFFhFp").unwrap()
        );
        assert_eq!(
            'L',
            get_common_item("jqHRNqRjqzjGDLG", "LrsFMfFZSrLrFZsSL").unwrap()
        );
        assert_eq!('P', get_common_item("PmmdzqPrV", "vPwwTWBwg").unwrap());
        assert_eq!(
            'v',
            get_common_item("wMqvLMZHhHMvwLH", "jbvcjnnSBnvTQFn").unwrap()
        );
        assert_eq!('t', get_common_item("ttgJtRGJ", "QctTZtZT").unwrap());
        assert_eq!(
            's',
            get_common_item("CrZsJsPPZsGz", "wwsLwLmpwMDw").unwrap()
        );
    }
}
