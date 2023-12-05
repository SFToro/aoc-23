use std::process::Output;

use nom::{
    bytes::complete::tag,
    character::complete::line_ending,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};
#[derive(Debug)]
struct Beacon(i32, i32);
#[derive(Debug)]
struct Sensor(i32, i32);

fn parse_beacon(input: &str) -> IResult<&str, Beacon> {
    let (input, (x, y)) = separated_pair(
        preceded(tag("beacon is at x="), nom::character::complete::i32),
        tag(","),
        preceded(tag(" y="), nom::character::complete::i32),
    )(input)?;
    Ok((input, Beacon(x, y)))
}
fn parse_sensor(input: &str) -> IResult<&str, Sensor> {
    let (input, (x, y)) = separated_pair(
        preceded(tag("Sensor at x="), nom::character::complete::i32),
        tag(","),
        preceded(tag(" y="), nom::character::complete::i32),
    )(input)?;
    Ok((input, Sensor(x, y)))
}
fn parse_line(input: &str) -> IResult<&str, (Sensor, Beacon)> {
    let (input, (sensor, beacon)) =
        separated_pair(parse_sensor, tag(": closest "), parse_beacon)(input)?;

    Ok((input, (sensor, beacon)))
}
fn parse_coords(input: &str) -> IResult<&str, Vec<(Sensor, Beacon)>> {
    let (input, output) = separated_list1(line_ending, parse_line)(input)?;
    Ok((input, output))
}
pub fn process_part_1(input: &str) -> usize {
    let (_, output) = parse_coords(input).unwrap();
    dbg!(output);
    todo!()
}
pub fn process_part_2(_input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
    #[test]
    fn it_works_part1() {
        let result = process_part_1(INPUT);
        assert_eq!(result, 31);
    }
    #[test]
    #[ignore]
    fn it_works_part2() {
        let result = process_part_2(INPUT);
        assert_eq!(result, 29);
    }
}
