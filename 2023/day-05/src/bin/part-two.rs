use rayon::prelude::*;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space1},
    multi::{count, many1, separated_list0, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

fn main() {
    let input: &str = include_str!("../../input.txt");
    if input.is_empty() {
        panic!("Input is empty");
    }
    println!("{}", process(input));
}

fn process(input: &str) -> u64 {
    let (_, map) = parse_map(input).unwrap();
    let starting_seeds = map.seeds.clone();

    starting_seeds
        .into_par_iter()
        .map(|(seed_initial, range)| -> u64 {
            let min = (seed_initial..(seed_initial + range))
                .into_par_iter()
                .map(|seed| {
                    let mut i = seed;
                    for block in map.blocks.iter() {
                        i = block.find_destination(i);
                    }
                    i
                })
                .min()
                .unwrap();
            min
        })
        .min()
        .unwrap()
}
fn parse_block(input: &str) -> IResult<&str, Block> {
    let (input, (start, end)) = separated_pair(alpha1, tag("-to-"), alpha1)(input)?;
    let (input, _) = tag(" map:")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, data) = separated_list1(
        line_ending,
        separated_list1(space1, nom::character::complete::u64),
    )(input)?;

    let data = data
        .into_iter()
        .map(|inner_vec| Coords {
            destination: inner_vec[0],
            source: inner_vec[1],
            range: inner_vec[2],
        })
        .collect::<Vec<_>>();

    Ok((
        input,
        Block {
            _start: start.into(),
            _end: end.into(),
            data,
        },
    ))
}

#[derive(Debug)]
struct Coords {
    destination: u64,
    source: u64,
    range: u64,
}

#[derive(Debug)]
struct Block {
    _start: String,
    _end: String,
    data: Vec<Coords>,
}

impl Block {
    fn find_destination(&self, i: u64) -> u64 {
        let mut destination: Option<u64> = None;
        for line in self.data.iter() {
            if i >= line.source && (i <= line.source + line.range) {
                let dif = i - line.source;
                destination = Some(line.destination + dif);
                break;
            }
        }
        if let Some(n) = destination {
            n
        } else {
            i
        }
    }
}

#[derive(Debug)]
struct Map {
    seeds: Vec<(u64, u64)>,
    blocks: Vec<Block>,
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = terminated(
        separated_list0(
            space1,
            separated_pair(
                nom::character::complete::u64,
                nom::character::complete::space1,
                nom::character::complete::u64,
            ),
        ),
        many1(line_ending),
    )(input)?;

    let (_, blocks) = separated_list1(count(line_ending, 2), parse_block)(input)?;

    Ok(("", Map { seeds, blocks }))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part_one() {
        assert_eq!(process(INPUT_TEXT), 46);
    }
}
