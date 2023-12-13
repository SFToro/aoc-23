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
    let mut rows_idx = Vec::new();
    let mut columns_idx = Vec::new();

    input
        .split("\n\n")
        .enumerate()
        .map(|(k, b)| {
            let mut pos: (isize, isize) = (-1, -1);
            let rows = b
                .lines()
                .enumerate()
                .skip(1)
                .filter_map(|(i, l)| {
                    if l == b.lines().nth(i - 1).unwrap() {
                        Some(i - 1)
                    } else {
                        None
                    }
                })
                .find_map(|il| {
                    let res = check_row_reflection(b, il).then_some(100 * (il + 1));
                    if res.is_some() {
                        rows_idx.push((k, il))
                    }
                    res
                });

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
                .filter_map(|(j, l)| {
                    if l == transposed_lines.iter().nth(j - 1).unwrap() {
                        Some((j - 1))
                    } else {
                        None
                    }
                })
                .find_map(|il| {
                    let res =
                        check_row_reflection(transposed_block.as_str(), il).then_some((il + 1));
                    if res.is_some() {
                        columns_idx.push((k, il))
                    }
                    res
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

fn check_row_reflection(block: &str, line_number: usize) -> bool {
    let mut above: Vec<_> = block.lines().take(line_number + 1).collect();
    let below = block.lines().skip(line_number + 1);
    above.reverse();

    below.zip(above.iter()).all(|(l1, l2)| l1 == *l2)
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
    const INPUT_TEXT2: &str = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_part_one() {
        assert_eq!(process(INPUT_TEXT), 405);
    }
}
