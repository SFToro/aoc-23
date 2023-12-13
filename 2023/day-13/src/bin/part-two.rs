use core::panic;

use nom::bytes::complete::{tag, take_until, take_until1};
use nom::character::complete::char;
use nom::combinator::all_consuming;
use nom::multi::{many1, separated_list0};
use nom::{
    branch::alt,
    character::complete::line_ending,
    multi::{count, separated_list1},
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
    input
        .split("\n\n")
        .enumerate()
        .map(|(k, b)| {
            let mut pos: (isize, isize) = (-1, -1);
            let rows = b
                .lines()
                .enumerate()
                .skip(1)
                .find_map(|(i, _)| find_smuldge_reflection(b, i).then_some(100 * (i)));

            if let Some(r) = rows {
                return r;
            }

            let chars: Vec<_> = b.lines().map(|l| l.chars().collect::<Vec<_>>()).collect();

            let transposed_lines: Vec<_> = transpose(&chars)
                .iter()
                .map(|vec| vec.iter().collect::<String>())
                .collect();

            let transposed_block: String = transposed_lines.clone().join("\n");

            let columns = transposed_lines
                .iter()
                .enumerate()
                .skip(1)
                .find_map(|(i, _)| {
                    find_smuldge_reflection(transposed_block.as_str(), i).then_some((i))
                });
            if let Some(c) = columns {
                return c;
            }
            0
        })
        .sum::<usize>()
}

fn transpose(data: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let size_x = data.len();
    let size_y = data[0].len();
    let mut transposed = Vec::new();

    for j in 0..size_y {
        let mut temp = Vec::new();
        for i in 0..size_x {
            temp.push(data[i][j])
        }
        transposed.push(temp);
    }
    transposed
}

fn find_smuldge_reflection(block: &str, line_number: usize) -> bool {
    let mut above = block.lines().take(line_number).collect::<Vec<_>>();
    let below = block.lines().skip(line_number);
    above.reverse();

    let ans: usize = below
        .zip(above.iter())
        .map(|(l1, l2)| {
            (l1.chars()
                .zip(l2.chars())
                .map(|(c1, c2)| if c1 == c2 { 0 } else { 1 })
                .sum::<usize>())
        })
        .sum();
    ans == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_part_two() {
        assert_eq!(process(INPUT_TEXT), 400);
    }
}
