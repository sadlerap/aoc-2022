//! --- Day 2: Rock Paper Scissors ---
//!
//! The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.
//!
//! Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.
//!
//! Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.
//!
//! The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.
//!
//! The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
//!
//! Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.
//!
//! For example, suppose you were given the following strategy guide:
//!
//! A Y
//! B X
//! C Z
//!
//! This strategy guide predicts and recommends the following:
//!
//!  - In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
//!  - In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
//!  - The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
//!
//! In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).
//!
//! What would your total score be if everything goes exactly according to your strategy guide?

use aoc_runner_derive::aoc;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::space1, combinator::map,
    sequence::tuple, IResult,
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Throw {
    Rock,
    Paper,
    Scissors,
}

impl Throw {
    /// Parses what our opponent will throw for that round.
    /// ```rust
    /// # use aoc_2022::day2::Throw;
    /// assert_eq!(Throw::parse_theirs("A"), Ok(("", Throw::Rock)));
    /// assert_eq!(Throw::parse_theirs("B"), Ok(("", Throw::Paper)));
    /// assert_eq!(Throw::parse_theirs("C"), Ok(("", Throw::Scissors)));
    /// ```
    pub fn parse_theirs(input: &str) -> IResult<&str, Self> {
        alt((
            map(tag("A"), |_| Throw::Rock),
            map(tag("B"), |_| Throw::Paper),
            map(tag("C"), |_| Throw::Scissors),
        ))(input)
    }

    /// Parses what we should throw for that round.
    /// ```rust
    /// # use aoc_2022::day2::Throw;
    /// assert_eq!(Throw::parse_ours("X"), Ok(("", Throw::Rock)));
    /// assert_eq!(Throw::parse_ours("Y"), Ok(("", Throw::Paper)));
    /// assert_eq!(Throw::parse_ours("Z"), Ok(("", Throw::Scissors)));
    /// ```
    pub fn parse_ours(input: &str) -> IResult<&str, Self> {
        alt((
            map(tag("X"), |_| Throw::Rock),
            map(tag("Y"), |_| Throw::Paper),
            map(tag("Z"), |_| Throw::Scissors),
        ))(input)
    }
}

pub enum Outcome {
    Win,
    Lose,
    Draw
}

impl Outcome {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(tag("X"), |_| Outcome::Lose),
            map(tag("Y"), |_| Outcome::Draw),
            map(tag("Z"), |_| Outcome::Win),
        ))(input)
    }
}

pub struct Round {
    pub theirs: Throw,
    pub ours: Throw,
}

impl Round {
    fn parse_1(input: &str) -> IResult<&str, Self> {
        map(
            tuple((Throw::parse_theirs, space1, Throw::parse_ours)),
            |(theirs, _, ours)| Round { theirs, ours },
        )(input)
    }

    fn parse_2(input: &str) -> IResult<&str, Self> {
        map(
            tuple((Throw::parse_theirs, space1, Outcome::parse)),
            |(theirs, _, outcome)| {
                let ours = match (theirs, outcome) {
                    (Throw::Rock, Outcome::Win) => Throw::Paper,
                    (Throw::Rock, Outcome::Lose) => Throw::Scissors,
                    (Throw::Rock, Outcome::Draw) => Throw::Rock,
                    (Throw::Paper, Outcome::Win) => Throw::Scissors,
                    (Throw::Paper, Outcome::Lose) => Throw::Rock,
                    (Throw::Paper, Outcome::Draw) => Throw::Paper,
                    (Throw::Scissors, Outcome::Win) => Throw::Rock,
                    (Throw::Scissors, Outcome::Lose) => Throw::Paper,
                    (Throw::Scissors, Outcome::Draw) => Throw::Scissors,
                };
                Round { theirs, ours }
            }
        )(input)
    }

    /// Scores a given round.
    /// ```rust
    /// # use aoc_2022::day2::{Round, Throw};
    /// // a winning round
    /// let round = Round { theirs: Throw::Rock, ours: Throw::Paper };
    /// assert_eq!(round.score(), 8)
    /// ```
    pub fn score(&self) -> u32 {
        let selected_shape_score = match self.ours {
            Throw::Rock => 1,
            Throw::Paper => 2,
            Throw::Scissors => 3,
        };
        let outcome_score = match (self.theirs, self.ours) {
            (Throw::Rock, Throw::Rock) => 3,
            (Throw::Rock, Throw::Paper) => 6,
            (Throw::Rock, Throw::Scissors) => 0,
            (Throw::Paper, Throw::Rock) => 0,
            (Throw::Paper, Throw::Paper) => 3,
            (Throw::Paper, Throw::Scissors) => 6,
            (Throw::Scissors, Throw::Rock) => 6,
            (Throw::Scissors, Throw::Paper) => 0,
            (Throw::Scissors, Throw::Scissors) => 3,
        };
        selected_shape_score + outcome_score
    }
}

/// Simulates a set of rounds according to the rules of part 1.  Example:
/// ```rust
/// # use aoc_2022::day2::*;
/// let input = "A Y\nB X\nC Z";
/// assert_eq!(part1(input), 15);
/// ```
#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(Round::parse_1)
        .filter_map(|r| match r {
            Ok((_, round)) => Some(round),
            Err(_) => None,
        })
        .map(|r| r.score())
        .sum()
}

/// Simulates a set of rounds according to the rules of part 2.  Example:
/// ```rust
/// # use aoc_2022::day2::*;
/// let input = "A Y\nB X\nC Z";
/// assert_eq!(part2(input), 12);
/// ```
#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(Round::parse_2)
        .filter_map(|r| match r {
            Ok((_, round)) => Some(round),
            Err(_) => None,
        })
        .map(|r| r.score())
        .sum()
}
