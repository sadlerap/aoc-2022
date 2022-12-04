use aoc_runner_derive::aoc;

/// Finds the first common ascii character in two strings.
/// ```rust
/// # use aoc_2022::day3::*;
/// assert_eq!(common_element("abc", "cde"), 'c');
/// assert_eq!(common_element("bcd", "def"), 'd');
/// assert_eq!(common_element("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", "ttgJtRGJQctTZtZT"), 'Z');
/// ```
pub fn common_element(a: &str, b: &str) -> char {
    for x in a.bytes() {
        for y in b.bytes() {
            if x == y {
                return char::from(x);
            }
        }
    }
    unreachable!()
}

/// Finds a common character in all three strings.
/// ```rust
/// # use aoc_2022::day3::*;
/// assert_eq!(
///     common_element_2(
///         "vJrwpWtwJgWrhcsFMMfFFhFp",
///         "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
///         "PmmdzqPrVvPwwTWBwg"
///     ),
///     'r'
/// );
/// assert_eq!(
///     common_element_2(
///         "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
///         "ttgJtRGJQctTZtZT",
///         "CrZsJsPPZsGzwwsLwLmpwMDw"
///     ),
///     'Z'
/// );
/// ```
pub fn common_element_2(a: &str, b: &str, c: &str) -> char {
    for x in a.bytes() {
        for y in b.bytes() {
            if x == y {
                let x = char::from(x);
                if c.contains(x) {
                    return x;
                }
            }
        }
    }
    unreachable!()
}

/// Calculates the priority of a given character, if it has one.
/// ```rust
/// # use aoc_2022::day3::*;
/// assert_eq!(priority('a'), Ok(1));
/// assert_eq!(priority('z'), Ok(26));
/// assert_eq!(priority('A'), Ok(27));
/// assert_eq!(priority('Z'), Ok(52));
/// assert!(priority(';').is_err());
/// ```
pub fn priority(item: char) -> Result<u32, String> {
    let char_as_num: u32 = item.into();

    if u32::from(b'a') <= char_as_num && u32::from(b'z') >= char_as_num {
        Ok(char_as_num - u32::from(b'a') + 1)
    } else if u32::from(b'A') <= char_as_num && u32::from(b'Z') >= char_as_num {
        Ok(char_as_num - u32::from(b'A') + 1 + 26)
    } else {
        Err(format!("{item} does not have a priority"))
    }
}

/// Calculates the sum of the priorities of overlapping items in each rucksack.
/// ```rust
/// # use aoc_2022::day3::*;
/// let input = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
///             jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
///             PmmdzqPrVvPwwTWBwg\n\
///             wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
///             ttgJtRGJQctTZtZT\n\
///             CrZsJsPPZsGzwwsLwLmpwMDw";
/// assert_eq!(part1(input), 157);
/// ```
#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|s| {
            let split_len = s.len() / 2;
            common_element(&s[0..split_len], &s[split_len..])
        })
        .filter_map(|c| {
            if let Ok(x) = priority(c) {
                Some(x)
            } else {
                None
            }
        })
        .sum()
}

/// Calculates the sum of the priorities of overlapping items in each rucksack.
/// ```rust
/// # use aoc_2022::day3::*;
/// let input = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
///             jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
///             PmmdzqPrVvPwwTWBwg\n\
///             wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
///             ttgJtRGJQctTZtZT\n\
///             CrZsJsPPZsGzwwsLwLmpwMDw";
/// assert_eq!(part1(input), 157);
/// ```
#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    let mut lines_iter = input.lines();
    std::iter::from_fn(|| {
        let a = lines_iter.next()?;
        let b = lines_iter.next()?;
        let c = lines_iter.next()?;
        Some([a, b, c])
    })
    .map(|s| common_element_2(s[0], s[1], s[2]))
    .filter_map(|c| {
        if let Ok(x) = priority(c) {
            Some(x)
        } else {
            None
        }
    })
    .sum()
}
