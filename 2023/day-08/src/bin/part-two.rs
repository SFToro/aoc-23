use num_integer::lcm; // Import the lcm function from the num-integer crate

use std::collections::BTreeMap;

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

    map.iter()
        .filter(|(pos, (_, _))| pos.ends_with('A'))
        .map(|(pos, (_, _))| pos)
        .map(|node| {
            let mut current_node = node;

            let cycle_length = instructions.chars().cycle().enumerate().find_map(|(i, c)| {
                let new_node = match c {
                    'R' => &map.get(current_node).expect("Position not found in map").1,

                    'L' => &map.get(current_node).expect("Position not found in map").0,
                    _ => panic!("Unknown movement"),
                };

                if new_node.ends_with('Z') {
                    Some(i + 1)
                } else {
                    current_node = new_node;
                    None
                }
            });
            if let Some(cycle_length) = cycle_length {
                cycle_length
            } else {
                panic!("No cycle found");
            }
        })
        .fold(1, lcm)
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
