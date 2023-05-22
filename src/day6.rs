use std::collections::HashMap;

use aoc_runner_derive::aoc;

pub fn find_chars<const N: usize>(input: &str) -> anyhow::Result<u32> {
    let mut counter = HashMap::new();
    for c in input.chars().take(N) {
        *counter.entry(c).or_default() += 1
    }
    if counter.len() == N {
        return Ok(N as u32);
    }

    for ((first, last), count) in input.chars().zip(input.chars().skip(N)).zip((N + 1)..) {
        let x: &mut u32 = counter.get_mut(&first).unwrap();
        *x -= 1;
        if *x == 0 {
            counter.remove(&first);
        }

        *counter.entry(last).or_default() += 1;
        if counter.len() == N {
            return Ok(count as u32);
        }
    }

    anyhow::bail!("Failed to find unique sequential chars of length {N}");
}

/// Finds the first four characters that are sequentially distinct.  Returns the index of the last
/// distinct character.
///
/// ```rust
/// # use aoc_2022::day6::*;
/// assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap(), 7);
/// assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), 5);
/// assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), 6);
/// assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(), 10);
/// assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(), 11);
/// ```
#[aoc(day6, part1)]
pub fn part1(input: &str) -> anyhow::Result<u32> {
    find_chars::<4>(input)
}

/// Finds the first fourteen characters that are sequentially distinct.  Returns the index of the
/// last distinct character.
///
/// ```rust
/// # use aoc_2022::day6::*;
/// assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap(), 19);
/// assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), 23);
/// assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), 23);
/// assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(), 29);
/// assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(), 26);
/// ```
#[aoc(day6, part2)]
pub fn part2(input: &str) -> anyhow::Result<u32> {
    find_chars::<14>(input)
}
