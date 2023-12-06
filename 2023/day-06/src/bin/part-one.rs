use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, multispace1, space0, space1},
    multi::{many1, separated_list1},
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
    let (_, pairs) = parse_input(input).unwrap();
    pairs
        .iter()
        .map(|pair| {
            let mut range_start =
                pair.time as f64 - ((pair.time.pow(2) - 4 * pair.distance) as f64).sqrt();
            range_start = range_start / 2.0;

            let mut range_end =
                pair.time as f64 + ((pair.time.pow(2) - 4 * pair.distance) as f64).sqrt();
            range_end = range_end / 2.0;

            // Case they are integers
            if range_start.fract() < 1e-10 {
                range_start = (range_start.floor()) + 1.0;
            }
            if range_start.fract() < 1e-10 {
                range_end = range_end.floor() - 1.0;
            }

            let count = (range_start.ceil() as usize..=(range_end.floor() as usize)).count();
            count
        })
        .product()
}

#[derive(Debug)]
struct Pair {
    time: u64,
    distance: u64,
}
fn parse_input(input: &str) -> IResult<&str, Vec<Pair>> {
    let (input, _) = tag("Time:")(input)?;
    let (input, _) = many1(space1)(input)?;

    let (input, time) = separated_list1(many1(space1), nom::character::complete::u64)(input)?;

    let (input, _) = line_ending(input)?;
    let (input, _) = tag("Distance:")(input)?;
    let (input, _) = many1(space1)(input)?;
    let (input, distance) = separated_list1(multispace1, nom::character::complete::u64)(input)?;
    let pairs = time
        .into_iter()
        .zip(distance.into_iter())
        .map(|(time, distance)| Pair { time, distance })
        .collect();

    Ok((input, pairs))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part_one() {
        assert_eq!(process(INPUT_TEXT), 288);
    }
}
