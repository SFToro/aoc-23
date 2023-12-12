use std::str::Bytes;

use itertools::{repeat_n, Itertools};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until1},
    character::{self, complete::line_ending},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    let input: &str = include_str!("../../input.txt");
    if input.is_empty() {
        panic!("Input is empty");
    }
    println!("{}", process(input));
}

fn process(input: &str) -> usize {
    let (_, parsed) = parse_input(input).unwrap();

    parsed
        .into_par_iter()
        .map(|(pattern, counts)| get_combinations(pattern, counts))
        .sum()
}

fn parse_input(input: &str) -> IResult<&str, Vec<(&str, Vec<u64>)>> {
    let (input, parsed) = separated_list1(
        line_ending,
        separated_pair(
            take_until1(" "),
            tag(" "),
            separated_list1(tag(","), character::complete::u64),
        ),
    )(input)?;

    Ok((input, parsed))
}

fn get_combinations(pattern: &str, counts: Vec<u64>) -> usize {
    let interrogations = pattern.chars().filter(|c| *c == '?').count();
    let mut options = repeat_n(["#", "."].into_iter(), interrogations)
        .multi_cartesian_product()
        .map(|c| c.join(""));

    options
        .filter_map(|o| {
            let mut o = o.chars();
            let a: Vec<_> = pattern
                .chars()
                .map(|(c)| match c {
                    '?' => o.next().unwrap(),
                    c => c,
                })
                .group_by(|c| *c == '#')
                .into_iter()
                .filter_map(|(is_valid_group, x)| {
                    is_valid_group.then_some(x.into_iter().count() as u64)
                })
                .collect();
            a.iter().eq(counts.iter()).then_some(1)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_part_one() {
        assert_eq!(process(INPUT_TEXT), 21);
    }
}
