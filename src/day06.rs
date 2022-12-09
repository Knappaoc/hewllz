use std::{collections::HashSet, io::Read};

use anyhow::{anyhow, Result};

pub fn main(input: impl Read) -> Result<String> {
    main_02(input)
}

fn main_01(mut input: impl Read) -> Result<String> {
    run::<4>(input)
}

fn main_02(mut input: impl Read) -> Result<String> {
    run::<14>(input)
}

fn run<const N: usize>(mut input: impl Read) -> Result<String> {
    let mut s = String::new();
    input.read_to_string(&mut s);
    for (i, arr) in s.as_bytes().array_windows::<N>().enumerate() {
        let set: HashSet<_> = arr.into_iter().collect();
        if set.len() == N {
            println!("{}", std::str::from_utf8(&*arr)?);
            return Ok((i + N).to_string());
        }
    }
    Err(anyhow!("no start sequence found"))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::{main_01, main_02};

    #[test]
    fn test_main_01() {
        let file = indoc!("bvwbjplbgvbhsrlpgdmjqwftvncz");
        let res = main_01(file.as_bytes()).unwrap();
        assert_eq!(&*res, "5");
        let file = indoc!("nppdvjthqldpwncqszvftbrmjlhg");
        let res = main_01(file.as_bytes()).unwrap();
        assert_eq!(&*res, "6");
        let file = indoc!("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        let res = main_01(file.as_bytes()).unwrap();
        assert_eq!(&*res, "10");
        let file = indoc!("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        let res = main_01(file.as_bytes()).unwrap();
        assert_eq!(&*res, "11");
    }
    #[test]
    fn test_main_02() {
        let file = indoc!("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        let res = main_02(file.as_bytes()).unwrap();
        assert_eq!(&*res, "19");
        let file = indoc!("bvwbjplbgvbhsrlpgdmjqwftvncz");
        let res = main_02(file.as_bytes()).unwrap();
        assert_eq!(&*res, "23");
        let file = indoc!("nppdvjthqldpwncqszvftbrmjlhg");
        let res = main_02(file.as_bytes()).unwrap();
        assert_eq!(&*res, "23");
        let file = indoc!("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        let res = main_02(file.as_bytes()).unwrap();
        assert_eq!(&*res, "29");
        let file = indoc!("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        let res = main_02(file.as_bytes()).unwrap();
        assert_eq!(&*res, "26");
    }
}
