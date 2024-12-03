use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while_m_n},
    combinator::{map, map_res},
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult,
};

enum Instruction {
    Do,
    Mul(u32, u32),
    DoNot,
}

fn from_dec(input: &str) -> Result<u32, ParseIntError> {
    input.parse::<u32>()
}

fn is_dec_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn mul_digit(input: &str) -> IResult<&str, u32> {
    map_res(take_while_m_n(1, 3, is_dec_digit), from_dec)(input)
}

fn parse_mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;

    let (input, (a, b)) = delimited(
        tag("("),
        separated_pair(mul_digit, tag(","), mul_digit),
        tag(")"),
    )(input)?;

    Ok((input, Instruction::Mul(a, b)))
}

fn parse_do(input: &str) -> IResult<&str, Instruction> {
    map(tag("do()"), |_| Instruction::Do)(input)
}

fn parse_donot(input: &str) -> IResult<&str, Instruction> {
    map(tag("don't()"), |_| Instruction::DoNot)(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_mul, parse_do, parse_donot))(input)
}

fn parse_empty(input: &str) -> IResult<&str, ()> {
    map(take(1usize), |_| ())(input)
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Instruction> {
    let mut parse_valid_instructions =
        many1(map(many_till(parse_empty, parse_instruction), |(_, i)| i));

    let (_, instructions) = parse_valid_instructions(input).unwrap();

    instructions
}

struct State {
    mul: bool,
    acc: u32,
}

impl State {
    fn new() -> State {
        State { mul: true, acc: 0 }
    }

    fn execute(self, instruction: &Instruction) -> Self {
        match instruction {
            Instruction::Do => Self { mul: true, ..self },
            Instruction::Mul(a, b) if self.mul => Self {
                acc: self.acc + (a * b),
                ..self
            },
            Instruction::DoNot => Self { mul: false, ..self },
            _ => self,
        }
    }

    fn result(self) -> u32 {
        self.acc
    }
}

#[aoc(day3, part1)]
fn part1(input: &[Instruction]) -> u32 {
    input
        .iter()
        .filter_map(|i| match i {
            Instruction::Do => None,
            Instruction::Mul(a, b) => Some(Instruction::Mul(*a, *b)),
            Instruction::DoNot => None,
        })
        .fold(State::new(), |acc, i| acc.execute(&i))
        .result()
}

#[aoc(day3, part2)]
fn part2(input: &[Instruction]) -> u32 {
    input
        .iter()
        .fold(State::new(), |acc, i| acc.execute(i))
        .result()
}
