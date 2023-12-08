use num_integer::lcm; // Import the lcm function from the num-integer crate

use std::{
    collections::{BTreeMap, HashMap},
    vec,
};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, line_ending},
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

    let map = lines
        .into_iter()
        .map(|line| (line.current_pos, (line.left, line.right)))
        .collect::<BTreeMap<_, _>>();

    let mut nodes: Vec<_> = map
        .iter()
        .filter(|(pos, (_, _))| pos.ends_with('A'))
        .map(|(pos, (_, _))| Some(pos))
        .collect();

    let mut instructions = instructions.chars().cycle();
    let mut visited: HashMap<usize, Vec<&str>> = nodes
        .iter()
        .enumerate()
        .map(|(idx, _)| (idx, vec![]))
        .collect();

    while !nodes.iter().all(|node| node.is_none()) {
        let direction = instructions.next();

        nodes = nodes
            .into_iter()
            .enumerate()
            .map(|(idx, node)| {
                if let Some(n) = node {
                    let new_node = match direction {
                        Some('R') => &map.get(n).expect("Position not found in map").1,

                        Some('L') => &map.get(n).expect("Position not found in map").0,
                        _ => panic!("Unknown movement"),
                    };
                    visited.entry(idx).and_modify(|v| v.push(new_node));

                    if new_node.ends_with('Z') {
                        None
                    } else {
                        Some(new_node)
                    }
                } else {
                    None
                }
            })
            .collect();
    }
    visited.into_values().map(|v| v.len()).fold(1, lcm)
}

#[derive(Debug)]
struct Line<'a> {
    current_pos: &'a str,
    left: &'a str,
    right: &'a str,
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, (current_position, (left, right))) = separated_pair(
        alphanumeric1,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(alphanumeric1, tag(", "), alphanumeric1),
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

    const INPUT_TEXT: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_part_two() {
        assert_eq!(process(INPUT_TEXT), 6);
    }
}
