use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{char, digit1},
    combinator::{map_res, opt},
    sequence::{preceded, tuple},
    IResult,
};
use std::env;

#[derive(Debug)]
pub struct Args {
    pub memory: Option<i32>,
    pub percent: Option<i32>,
}

fn parse_i32(input: &str) -> IResult<&str, i32> {
    map_res(digit1, |s: &str| s.parse::<i32>())(input)
}

fn parse_memory(input: &str) -> IResult<&str, Option<i32>> {
    let (input, _) = alt((tag("--memory"), tag("-m")))(input)?;
    let (input, _) = opt(char(' '))(input)?;
    let (input, memory) = opt(parse_i32)(input)?;
    Ok((input, memory))
}

fn parse_percent(input: &str) -> IResult<&str, Option<i32>> {
    let (input, _) = alt((tag("--percent"), tag("-p")))(input)?;
    let (input, _) = opt(char(' '))(input)?;
    let (input, percent) = opt(parse_i32)(input)?;
    Ok((input, percent))
}

fn parse_arg(input: &str) -> IResult<&str, Args> {
    let (input, _) = take_while(|c| c == ' ')(input)?;
    let (input, memory) = opt(parse_memory)(input)?;
    let (input, _) = take_while(|c| c == ' ')(input)?;
    let (input, percent) = opt(parse_percent)(input)?;
    Ok((
        input,
        Args {
            memory: memory.flatten(),
            percent: percent.flatten(),
        },
    ))
}

pub fn parse_and_validate_args() -> Args {
    let args: Vec<String> = env::args().collect();
    let input = args[1..].join(" ");
    let (_, args) = alt((parse_arg, preceded(tuple((parse_arg, char(' '))), parse_arg)))(&input).unwrap();
    args
}
