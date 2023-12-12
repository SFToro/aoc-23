use std::{collections::HashMap, str::Bytes};

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

    let mut cache: HashMap<(&str, &[usize]), usize> = HashMap::new();
    parsed
        .iter()
        .map(|(pattern, counts)| get_combinations(&pattern, &counts[..], &mut cache))
        .sum()
}

fn parse_input(input: &str) -> IResult<&str, Vec<(String, Vec<usize>)>> {
    let (input, parsed) = separated_list1(
        line_ending,
        separated_pair(
            take_until1(" ").map(|s: &str| (s.to_string() + "?").repeat(4) + s),
            tag(" "),
            separated_list1(tag(","), character::complete::u64).map(|v| {
                v.repeat(5)
                    .into_iter()
                    .map(|n| n as usize)
                    .collect::<Vec<_>>()
            }),
        ),
    )(input)?;

    Ok((input, parsed))
}

fn get_combinations<'b, 'a>(
    pattern: &'b str,
    nums: &'a [usize],
    cache: &mut HashMap<(&'b str, &'a [usize]), usize>,
) -> usize {
    if pattern.is_empty() {
        return if nums.len() == 0 { 1 } else { 0 };
    };
    if nums.is_empty() {
        return if pattern.contains("#") { 0 } else { 1 };
    };

    if let Some(&v) = cache.get(&(&pattern, nums)) {
        return v;
    }

    let mut result = 0;

    if ".?".contains(pattern.chars().next().unwrap()) {
        result += get_combinations(&pattern[1..], nums, cache);
    }
    if "#?".contains(pattern.chars().next().unwrap()) {
        if let Some(&num) = nums.first() {
            if num <= pattern.len()
                && !pattern.get(..num).unwrap_or("").contains(".")
                && (nums[0] == pattern.len() || pattern[num..].chars().next().unwrap() != '#')
            {
                let new = pattern.get((nums[0] + 1)..).unwrap_or("");
                result += get_combinations(new, &nums[1..], cache)
            }
        }
    }
    cache.insert((&pattern, nums), result);
    result
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

    const INPUT_TEXT2: &str = "?###???????? 3,2,1";

    #[test]
    fn test_part_one() {
        assert_eq!(process(INPUT_TEXT), 525152);
        assert_eq!(process(INPUT_TEXT2), 506250);
    }
}
