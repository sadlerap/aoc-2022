use anyhow::{anyhow, bail, Result};
use aoc_runner_derive::aoc;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete,
    combinator::{map, opt},
    multi::{many0, separated_list1},
    sequence::{terminated, tuple},
    IResult,
};

fn parse_stacks(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (rest, stack_data) = take_until("\n\n")(input)?;

    let num_stacks = if let Some(Some(num_stacks)) = stack_data.lines().last().map(|s| {
        s.split_whitespace()
            .last()
            .map(|s| s.parse::<u32>().unwrap_or_default())
    }) {
        dbg!(num_stacks)
    } else {
        0
    };
    let mut stacks = vec![vec![]; num_stacks as usize];
    for line in stack_data.lines() {
        if line.starts_with(" 1") {
            break;
        }
        let data = separated_list1(
            tag(" "),
            alt((
                map(tag("   "), |_| -> Option<char> { None }),
                map(
                    tuple((tag("["), complete::anychar, tag("]"))),
                    |(_, c, _)| -> Option<char> { Some(c) },
                ),
            )),
        )(line)?;
        data.1
            .iter()
            .zip(stacks.iter_mut())
            .filter_map(|x| {
                if let Some(c) = *x.0 {
                    Some((c, x.1))
                } else {
                    None
                }
            })
            .for_each(|(c, stack)| stack.push(c))
    }

    // we need to reverse our stacks; the elements we saw first need to come on top of the stack,
    // not the bottom.
    stacks.iter_mut().for_each(|v| v.reverse());

    Ok((rest, stacks))
}

#[derive(Debug)]
struct Command {
    amount: usize,
    from: usize,
    to: usize,
}

impl Command {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                tag("move "),
                complete::u32,
                tag(" from "),
                complete::u32,
                tag(" to "),
                complete::u32,
            )),
            |(_, amount, _, from, _, to)| Self {
                amount: amount as usize,
                from: from as usize - 1, // input is 1-indexed, we use 0
                to: to as usize - 1,
            },
        )(input)
    }
}

#[derive(Debug)]
struct Crane {
    stacks: Vec<Vec<char>>,
    commands: Vec<Command>,
}

impl Crane {
    fn parse(input: &str) -> IResult<&str, Crane> {
        map(
            tuple((
                parse_stacks,
                tag("\n\n"),
                many0(terminated(Command::parse, opt(complete::newline))),
            )),
            |(stacks, _, commands)| Crane { stacks, commands },
        )(input)
    }

    fn process_commands(&mut self) -> anyhow::Result<()> {
        for command in self.commands.iter() {
            for _ in 0..command.amount {
                let from_stack = &mut self.stacks[command.from];
                let Some(c) = from_stack.pop() else {
                    bail!("Attempted to remove a crate from empty stack {0}!", command.from);
                };
                let to_stack = &mut self.stacks[command.to];
                to_stack.push(c);
            }
        }

        Ok(())
    }

    fn process_commands_2(&mut self) -> anyhow::Result<()> {
        for command in self.commands.iter() {
            let from_stack = &mut self.stacks[command.from];
            let to_remove = from_stack.len().saturating_sub(command.amount);
            let data: Vec<_> = from_stack.drain(to_remove..).collect();
            let to_stack = &mut self.stacks[command.to];
            to_stack.extend_from_slice(&data);
        }

        Ok(())
    }

    fn read_stack_tops(&self) -> String {
        let mut s = String::new();
        for stack in self.stacks.iter() {
            if let Some(c) = stack.last() {
                s.push(*c);
            }
        }

        s
    }
}

#[aoc(day5, part1)]
fn part1(input: &str) -> Result<String> {
    let mut crane = Crane::parse(input)
        .map(|x| x.1)
        .map_err(|e| anyhow!("Failed to parse input: {e}"))?;

    crane.process_commands()?;
    Ok(crane.read_stack_tops())
}

#[aoc(day5, part2)]
fn part2(input: &str) -> Result<String> {
    let mut crane = Crane::parse(input)
        .map(|x| x.1)
        .map_err(|e| anyhow!("Failed to parse input: {e}"))?;

    crane.process_commands_2()?;
    Ok(crane.read_stack_tops())
}
