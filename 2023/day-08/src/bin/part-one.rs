use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

fn main() {
    let input: &str = include_str!("../../input.txt");
    if input.is_empty() {
        panic!("Input is empty");
    }
    println!("{}", process(input));
}

fn process(input: &str) -> usize {
    let (_, (instructions, lines)) = parse(input).expect("Failed to parse input");

    dbg!(instructions.chars().count());

    let map = lines
        .into_iter()
        .map(|line| (line.current_pos, (line.left, line.right)))
        .collect::<BTreeMap<_, _>>();

    let mut current_pos = "AAA";
    dbg!(current_pos);
    let mut steps = 0;
    'outer: loop {
        for char in instructions.chars() {
            match char {
                'R' => current_pos = map.get(current_pos).expect("Position not found in map").1,
                'L' => current_pos = map.get(current_pos).expect("Position not found in map").0,
                _ => panic!("Unknown movement"),
            };

            steps += 1;

            if current_pos == "ZZZ" {
                dbg!(steps);
                break 'outer;
            };
        }
    }

    steps
}

#[derive(Debug)]
struct Line<'a> {
    current_pos: &'a str,
    left: &'a str,
    right: &'a str,
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, (current_position, (left, right))) = separated_pair(
        alpha1,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(alpha1, tag(", "), alpha1),
            tag(")"),
        ),
    )(input)?;

    Ok((
        input,
        Line {
            current_pos: current_position,
            left,
            right,
        },
    ))
}

fn parse(input: &str) -> IResult<&str, (&str, Vec<Line<'_>>)> {
    let (input, instructions) = alpha1(input)?;
    let (input, _) = many1(line_ending)(input)?;
    let (input, lines) = separated_list1(line_ending, parse_line)(input)?;

    Ok((input, (instructions, lines)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_part_one() {
        assert_eq!(process(INPUT_TEXT), 2);
    }
    const INPUT_TEXT2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_part_one2() {
        assert_eq!(process(INPUT_TEXT2), 6);
    }
}
