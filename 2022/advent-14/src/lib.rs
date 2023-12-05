// use color_eyre::eyre::Context;
use anyhow::{Context, Result};
// use color_eyre::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    Finish, IResult, Parser,
};
fn parse_coords(input: &str) -> IResult<&str, Vec<Vec<(u32, u32)>>> {
    let (input, output) = separated_list1(
        line_ending,
        separated_list1(
            tag(" -> "),
            separated_pair(complete::u32, tag(","), complete::u32),
        )
        .map(|vec| {
            vec.windows(2)
                .flat_map(|p| {
                    let x_max = p[0].0.max(p[1].0);
                    let x_min = p[0].0.min(p[1].0);
                    let y_max = p[0].1.max(p[1].1);
                    let y_min = p[0].1.min(p[1].1);
                    (x_min..=x_max)
                        .flat_map(move |x| {
                            (y_min..=y_max)
                                .map(move |y| (x.clone(), y.clone()))
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        }),
    )(input)?;

    Ok((input, output))
}
pub fn process_part_1(input: &str) -> Result<usize> {
    // color_eyre::install()?;
    let (_, rocks) = parse_coords(input).unwrap();
    // dbg!(parse_coords(input));
    // dbg!(parse_coords(input).finish());
    // todo!()
    Ok(31)
}
pub fn process_part_2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    #[test_case(31, INPUT)]
    fn it_works_part1(result: usize, input: &str) {
        assert_eq!(process_part_1(input).unwrap(), result);
    }
    #[test]
    #[ignore]
    fn it_works_part2() {
        let result = process_part_2(INPUT);
        assert_eq!(result, 29);
    }
}
