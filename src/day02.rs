use anyhow::{anyhow, Context, Result};
use num_enum::TryFromPrimitive;
use std::{
    io::{BufRead, BufReader, Read},
    ops::{Add, Sub},
    str::FromStr,
};
use tap::Conv;

pub fn main(file: impl Read) -> Result<String> {
    main_02(file)
}
/// get the expected score, interpreting the lines as pairs of hands
fn main_01(file: impl Read) -> Result<String> {
    let ret = itertools::process_results(get_lines(file), |it| run_01(it))??;
    println!("{ret}");
    Ok(ret.to_string())
}
/// get the expected score, interpreting the lines as a hand and an expected outcome
fn main_02(file: impl Read) -> Result<String> {
    let ret = itertools::process_results(get_lines(file), |it| run_02(it))??;
    println!("{ret}");
    Ok(ret.to_string())
}
fn get_lines(file: impl Read) -> impl Iterator<Item = Result<(usize, String)>> {
    let lines = BufReader::new(file)
        .lines()
        .enumerate()
        .map(|(i, r)| -> Result<(usize, String)> {
            Ok((i, r.context(format!("failed to read line {i} of input"))?))
        });
    lines
}
/// get the expected score, interpreting the lines as pairs of hands
fn run_01(lines: impl Iterator<Item = (usize, String)>) -> Result<String> {
    let mut sum = 0;

    for (i, line) in lines {
        let pair: (Hand, Hand) = parse_01(&line).context(format!("could not process line {i}"))?;
        sum += pair.score();
    }
    Ok(format!("{sum}"))
}

/// get the expected score, interpreting the lines as a hand and an expected outcome
fn run_02(lines: impl Iterator<Item = (usize, String)>) -> Result<String> {
    let mut sum = 0;

    for (i, line) in lines {
        let pair: (Hand, Outcome) =
            parse_02(&line).context(format!("could not process line {i}"))?;
        sum += pair.score();
    }
    Ok(format!("{sum}"))
}

fn parse_01(line: &str) -> Result<(Hand, Hand)> {
    let (opp, me) = line.split_once(" ").context("could not split by space")?;
    let pair = (
        opp.parse().context("parsing first item")?,
        me.parse().context("parsing second item")?,
    );
    Ok(pair)
}
fn parse_02(line: &str) -> Result<(Hand, Outcome)> {
    let (opp, me) = line.split_once(" ").context("could not split by space")?;
    let pair = (
        opp.parse().context("parsing first item")?,
        me.parse().context("parsing second item")?,
    );
    Ok(pair)
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, TryFromPrimitive)]
#[repr(u8)]
enum Hand {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}
impl From<Modulo<3>> for Hand {
    fn from(value: Modulo<3>) -> Self {
        value.0.try_into().unwrap()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, TryFromPrimitive)]
#[repr(u8)]
enum Outcome {
    Win = 1,
    Draw = 0,
    Lose = 2,
}
impl From<Modulo<3>> for Outcome {
    fn from(value: Modulo<3>) -> Self {
        value.0.try_into().unwrap()
    }
}
impl Score for Outcome {
    fn score(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }
}
impl From<Outcome> for Modulo<3> {
    fn from(value: Outcome) -> Self {
        Self::new(value as u8)
    }
}
impl From<Hand> for Modulo<3> {
    fn from(value: Hand) -> Self {
        Self::new(value as u8)
    }
}

trait Score {
    fn score(&self) -> u32;
}

impl Score for (Hand, Hand) {
    fn score(&self) -> u32 {
        let (opponent, you) = self;
        let outcome = you.fight(*opponent);
        outcome.score() + you.score()
    }
}
impl Score for (Hand, Outcome) {
    fn score(&self) -> u32 {
        let (opp, outcome) = self;
        let you = opp.rig_against(*outcome);
        outcome.score() + you.score()
    }
}
impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissors),
            _ => Err(anyhow!("Invalid hand {s}")),
        }
    }
}
impl FromStr for Outcome {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(anyhow!("Invalid outcome {s}")),
        }
    }
}
impl Hand {
    // fn from_abc(c: &str) -> Option<Hand> {
    //     match c {
    //         "A" => Some(Hand::Rock),
    //         "B" => Some(Hand::Paper),
    //         "C" => Some(Hand::Scissors),
    //         _ => None,
    //     }
    // }
    // fn from_xyz(c: &str) -> Option<Hand> {
    //     match c {
    //         "X" => Some(Hand::Rock),
    //         "Y" => Some(Hand::Paper),
    //         "Z" => Some(Hand::Scissors),
    //         _ => None,
    //     }
    // }
    fn fight(self, other: Self) -> Outcome {
        let diff = self.conv::<Modulo<3>>() - other.conv::<Modulo<3>>();
        Outcome::from(diff)
    }

    /// if A = B.rig_against(O)
    /// A.fight(B) = O
    fn rig_against(self, outcome: Outcome) -> Hand {
        let diff = self.conv::<Modulo<3>>() + outcome.conv::<Modulo<3>>();
        Hand::from(diff)
    }
    /// if B = A.rig(O)
    /// A.fight(B) = O
    fn rig(self, outcome: Outcome) -> Hand {
        let diff = self.conv::<Modulo<3>>() - outcome.conv::<Modulo<3>>();
        Hand::from(diff)
    }
}
impl Score for Hand {
    fn score(&self) -> u32 {
        *self as u32 + 1
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Modulo<const N: u8>(u8);
impl<const N: u8> From<u8> for Modulo<N> {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}
impl<const N: u8> Modulo<N> {
    fn new(x: u8) -> Self {
        Self(x % N)
    }
}
impl<const N: u8> Sub for Modulo<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Modulo::new(N - rhs.0 + self.0)
    }
}
impl<const N: u8> Add for Modulo<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Modulo::new(rhs.0 + self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::day02::main_01;
    use crate::day02::main_02;
    use crate::day02::parse_02;

    use super::Hand;
    use super::Modulo;
    use super::Outcome;
    use super::Score;
    use indoc::indoc;

    #[test]
    fn test_main_01() {
        let input = indoc!(
            "A Y
            B X
            C Z"
        );
        let res = main_01(input.as_bytes()).unwrap();
        assert_eq!(&*res, "15")
    }
    #[test]
    fn test_main_02() {
        let input = indoc!(
            "A Y
            B X
            C Z"
        );
        let res = main_02(input.as_bytes()).unwrap();
        assert_eq!(&*res, "12")
    }
    #[test]
    fn test_full() {
        // assert_eq!(parse_line("A Y"), Some((Hand::Rock, Hand::Paper)));
        // assert_eq!(parse_line("B X"), Some((Hand::Paper, Hand::Rock)));
        // assert_eq!(parse_line("C Z"), Some((Hand::Scissors, Hand::Scissors)));
        assert_eq!(parse_02("A Y").unwrap(), (Hand::Rock, Outcome::Draw));
        assert_eq!(parse_02("B X").unwrap(), (Hand::Paper, Outcome::Lose));
        assert_eq!(parse_02("C Z").unwrap(), (Hand::Scissors, Outcome::Win));
    }
    #[test]
    fn test_rig() {
        assert_eq!(Hand::Rock.rig(Outcome::Win), Hand::Scissors);
        assert_eq!(Hand::Paper.rig(Outcome::Win), Hand::Rock);
        assert_eq!(Hand::Scissors.rig(Outcome::Win), Hand::Paper);

        assert_eq!(Hand::Scissors.rig(Outcome::Lose), Hand::Rock);
        assert_eq!(Hand::Rock.rig(Outcome::Lose), Hand::Paper);
        assert_eq!(Hand::Paper.rig(Outcome::Lose), Hand::Scissors);

        assert_eq!(Hand::Rock.rig(Outcome::Draw), Hand::Rock);
        assert_eq!(Hand::Paper.rig(Outcome::Draw), Hand::Paper);
        assert_eq!(Hand::Scissors.rig(Outcome::Draw), Hand::Scissors);
    }
    #[test]
    fn test_rig_against() {
        assert_eq!(Hand::Scissors.rig_against(Outcome::Win), Hand::Rock);
        assert_eq!(Hand::Rock.rig_against(Outcome::Win), Hand::Paper);
        assert_eq!(Hand::Paper.rig_against(Outcome::Win), Hand::Scissors);

        assert_eq!(Hand::Rock.rig_against(Outcome::Lose), Hand::Scissors);
        assert_eq!(Hand::Paper.rig_against(Outcome::Lose), Hand::Rock);
        assert_eq!(Hand::Scissors.rig_against(Outcome::Lose), Hand::Paper);

        assert_eq!(Hand::Rock.rig_against(Outcome::Draw), Hand::Rock);
        assert_eq!(Hand::Paper.rig_against(Outcome::Draw), Hand::Paper);
        assert_eq!(Hand::Scissors.rig_against(Outcome::Draw), Hand::Scissors);
    }
    #[test]
    fn test_fight_score() {
        assert_eq!((Hand::Rock, Hand::Paper).score(), 8);
        assert_eq!((Hand::Paper, Hand::Rock).score(), 1);
        assert_eq!((Hand::Scissors, Hand::Scissors).score(), 6);
        assert_eq!((Hand::Rock, Hand::Rock).score(), 4);
        assert_eq!((Hand::Rock, Outcome::Draw).score(), 4);
        assert_eq!((Hand::Paper, Hand::Rock).score(), 1);
        assert_eq!((Hand::Paper, Outcome::Lose).score(), 1);
        assert_eq!((Hand::Scissors, Hand::Rock).score(), 7);
    }
    #[test]
    fn test_score() {
        assert_eq!(Hand::Rock.score(), 1);
        assert_eq!(Hand::Paper.score(), 2);
        assert_eq!(Hand::Scissors.score(), 3);
        assert_eq!(Outcome::Win.score(), 6);
        assert_eq!(Outcome::Draw.score(), 3);
        assert_eq!(Outcome::Lose.score(), 0);
    }
    #[test]
    fn test_fights() {
        assert_eq!(Hand::Rock.fight(Hand::Scissors), Outcome::Win);
        assert_eq!(Hand::Paper.fight(Hand::Rock), Outcome::Win);
        assert_eq!(Hand::Scissors.fight(Hand::Paper), Outcome::Win);

        assert_eq!(Hand::Scissors.fight(Hand::Rock), Outcome::Lose);
        assert_eq!(Hand::Rock.fight(Hand::Paper), Outcome::Lose);
        assert_eq!(Hand::Paper.fight(Hand::Scissors), Outcome::Lose);

        assert_eq!(Hand::Rock.fight(Hand::Rock), Outcome::Draw);
        assert_eq!(Hand::Paper.fight(Hand::Paper), Outcome::Draw);
        assert_eq!(Hand::Scissors.fight(Hand::Scissors), Outcome::Draw);
    }
    #[test]
    fn test_modulo_new() {
        assert_eq!(Modulo::<3>::new(0), Modulo::<3>(0));
        assert_eq!(Modulo::<3>::new(1), Modulo::<3>(1));
        assert_eq!(Modulo::<3>::new(2), Modulo::<3>(2));
        assert_eq!(Modulo::<3>::new(3), Modulo::<3>(0));
        assert_eq!(Modulo::<3>::new(4), Modulo::<3>(1));
        assert_eq!(Modulo::<3>::new(5), Modulo::<3>(2));
    }
    #[test]
    fn test_modulo_sub() {
        assert_eq!(Modulo::<3>(0) - Modulo::<3>(0), Modulo::<3>(0));
        assert_eq!(Modulo::<3>(1) - Modulo::<3>(1), Modulo::<3>(0));
        assert_eq!(Modulo::<3>(2) - Modulo::<3>(2), Modulo::<3>(0));
        assert_eq!(Modulo::<3>(0) - Modulo::<3>(0), Modulo::<3>(0));
        assert_eq!(Modulo::<3>(1) - Modulo::<3>(0), Modulo::<3>(1));
        assert_eq!(Modulo::<3>(2) - Modulo::<3>(0), Modulo::<3>(2));
        assert_eq!(Modulo::<3>(0) - Modulo::<3>(0), Modulo::<3>(0));
        assert_eq!(Modulo::<3>(0) - Modulo::<3>(1), Modulo::<3>(2));
        assert_eq!(Modulo::<3>(0) - Modulo::<3>(2), Modulo::<3>(1));
    }
    #[test]
    fn test_modulo_add() {
        assert_eq!(Modulo::<3>(0) + Modulo::<3>(0), Modulo::<3>(0));
        assert_eq!(Modulo::<3>(1) + Modulo::<3>(1), Modulo::<3>(2));
        assert_eq!(Modulo::<3>(2) + Modulo::<3>(2), Modulo::<3>(1));
    }
}
