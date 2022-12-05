use anyhow::anyhow;
use aoc_runner_derive::aoc;
use nom::{bytes::complete::tag, character::complete, combinator::map, sequence::tuple, IResult};

#[derive(Debug)]
pub struct Range {
    pub start: u32,
    pub end: u32,
}

impl Range {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((complete::u32, tag("-"), complete::u32)),
            |(start, _, end)| Self { start, end },
        )(input)
    }

    /// Does this range contain the other range?
    pub fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    /// Does this range overlap with the other range at all?
    /// ```rust
    /// # use aoc_2022::day4::*;
    /// let r1 = Range {start: 1, end: 3};
    /// let r2 = Range {start: 3, end: 4};
    /// assert!(r1.overlaps(&r2));
    /// ```
    pub fn overlaps(&self, other: &Self) -> bool {
        let r1 = self.start..=self.end;
        let r2 = other.start..=other.end;
        r1.contains(&other.start)
            || r1.contains(&other.end)
            || r2.contains(&self.start)
            || r2.contains(&self.end)
    }
}

pub struct Assignment(Range, Range);

impl Assignment {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((Range::parse, tag(","), Range::parse)),
            |(first, _, second)| Assignment(first, second),
        )(input)
    }
}

/// Finds how many elves were assigned containing ranges for work.
///
/// ```rust
/// # use aoc_2022::day4::*;
/// let input = "2-4,6-8\n\
/// 2-3,4-5\n\
/// 5-7,7-9\n\
/// 2-8,3-7\n\
/// 6-6,4-6\n\
/// 2-6,4-8";
/// assert_eq!(part1(input).unwrap(), 2);
/// ```
#[aoc(day4, part1)]
pub fn part1(input: &str) -> anyhow::Result<u32> {
    let mut count = 0;
    for line in input.lines() {
        let assignment = Assignment::parse(line)
            .map(|x| x.1)
            .map_err(|e| anyhow!("Failed to parse line {line}: {e}"))?;
        if assignment.0.contains(&assignment.1) || assignment.1.contains(&assignment.0) {
            count += 1;
        }
    }

    Ok(count)
}

/// Finds how many elves were assigned overlapping ranges for work.
///
/// ```rust
/// # use aoc_2022::day4::*;
/// let input = "2-4,6-8\n\
///              2-3,4-5\n\
///              5-7,7-9\n\
///              2-8,3-7\n\
///              6-6,4-6\n\
///              2-6,4-8";
/// assert_eq!(part2(input).unwrap(), 4);
/// ```
#[aoc(day4, part2)]
pub fn part2(input: &str) -> anyhow::Result<u32> {
    let mut count = 0;
    for line in input.lines() {
        let assignment = Assignment::parse(line)
            .map(|x| x.1)
            .map_err(|e| anyhow!("Failed to parse line {line}: {e}"))?;
        if assignment.0.overlaps(&assignment.1) {
            count += 1;
        }
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlaps() {
        let r1 = Range::parse("2-4").unwrap().1;
        let r2 = Range::parse("4-6").unwrap().1;
        assert!(r1.overlaps(&r2));
        assert!(r2.overlaps(&r1));

        let r1 = Range::parse("2-4").unwrap().1;
        let r2 = Range::parse("3-3").unwrap().1;
        assert!(r1.overlaps(&r2));
        assert!(r2.overlaps(&r1));

        let r1 = Range::parse("2-2").unwrap().1;
        let r2 = Range::parse("2-3").unwrap().1;
        assert!(r1.overlaps(&r2));
        assert!(r2.overlaps(&r1));

        let r1 = Range::parse("1-5").unwrap().1;
        let r2 = Range::parse("2-3").unwrap().1;
        assert!(r1.overlaps(&r2));
        assert!(r2.overlaps(&r1));
    }
}
