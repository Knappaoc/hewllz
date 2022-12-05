use chumsky::{error::Cheap, prelude::*, text::int};
use std::{io::Read, str::FromStr};

use anyhow::Result;

pub fn main(input: impl Read) -> Result<String, !> {
    main_01(input)
}

fn main_01(_input: impl Read) -> Result<String, !> {
    todo!()
}

fn main_02(_input: impl Read) -> Result<String, !> {
    todo!()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}
impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Stacks(Vec<Vec<char>>);
impl FromStr for Stacks {
    type Err = Vec<Simple<char>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // A parser that parses any number of whitespace characters without allocating
        Self::parser().parse(s)
    }
}
impl Stacks {
    fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
        todo!()
    }
}
impl Instruction {
    fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
        let whitespace = filter::<_, _, Cheap<char>>(|c: &char| c.is_whitespace())
            .ignored()
            .repeated();
        let mov = text::keyword("move").ignored().padded();
        let from = just("from").padded();
        let to = just("to").ignored().padded();
        let int = text::int(10).map(|s: String| s.parse()).padded();
        /*
        let parser = mov
            .then(int)
            .map(|s: &str| s.parse())
            .padded()
            .then(from)
            .then(int)
            .padded()
            .then(to)
            .padded()
            .then(int)
            .padded()
            .map(|x| Instruction {
                from: 0,
                to: 0,
                count: 0,
            });
        parser
        */
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day05::{Instruction, Stacks};

    use super::{main_01, main_02};

    #[test]
    fn test_parse_instructions() {
        let file = "move 1 from 2 to 3";

        let expected = Instruction {
            count: 1,
            from: 2,
            to: 3,
        };
        assert_eq!(expected, file.parse().unwrap());
    }

    #[test]
    fn test_parse_stacks() {
        let file = indoc!(
            "    [D]    
            [N] [C]    
            [Z] [M] [P]
             1   2   3 "
        );

        let expected = Stacks(vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]);
        assert_eq!(expected, file.parse().unwrap());
    }

    #[test]
    fn test_main_01() {
        let file = indoc!(
            "    [D]    
            [N] [C]    
            [Z] [M] [P]
             1   2   3 

            move 1 from 2 to 1
            move 3 from 1 to 3
            move 2 from 2 to 1
            move 1 from 1 to 2"
        );
        let res = main_01(file.as_bytes()).unwrap();
        assert_eq!(&*res, "CMZ");
    }
    #[test]
    fn test_main_02() {
        let file = indoc!(
            "    [D]    
            [N] [C]    
            [Z] [M] [P]
             1   2   3 

            move 1 from 2 to 1
            move 3 from 1 to 3
            move 2 from 2 to 1
            move 1 from 1 to 2"
        );
        let res = main_02(file.as_bytes()).unwrap();
        assert_eq!(&*res, "");
    }
}
