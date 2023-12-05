use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::{map, value},
    multi::separated_list1,
    sequence::preceded,
    IResult, Parser,
};
#[derive(Clone, Copy, Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}
fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let noop = tag("noop");
    let addx = preceded(tag("addx "), nom::character::complete::i32);
    // alt((value(Instruction::Noop, noop), map(addx, Instruction::Addx)))(input)
    alt((
        noop.map(|_| Instruction::Noop),
        addx.map(|q| Instruction::Addx(q)),
    ))(input)
}
fn parse_file(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, instructions) = separated_list1(newline, parse_instruction)(input)?;
    Ok((input, instructions))
}
pub fn process_part1(input: &str) -> String {
    let (_, vec_instrs) = parse_file(input).unwrap();
    let mut prev2: &Instruction = &vec_instrs[0];
    let mut prev1: &Instruction = &vec_instrs[1];
    let mut i = 2;
    let mut registry: i32 = 1;
    let mut sum = 0;
    let mut last = 0;
    for (_, instr) in vec_instrs.iter().skip(2).enumerate() {
        if (i - 20) % 40 == 0 {
            sum += (i) * registry;
            last = i;
        } else if (i - 20) % 40 == 1 && (last) != i - 1 {
            sum += (i - 1) * registry;
            last = i;
        }
        match prev2 {
            Instruction::Addx(q) => {
                registry += q;
                i += 2
            }
            _ => i += 1,
        };
        prev2 = prev1;
        prev1 = instr;
    }
    sum.to_string()
}
pub fn process_part2(input: &str) -> String {
    let (_, vec_instrs) = parse_file(input).unwrap();
    let mut i = 0;
    let mut registry: i32 = 1;
    let mut last = 0;
    let mut crt_pixels: String = "".to_string();
    for instr in vec_instrs.iter() {
        // let mut pixel_id = 0;
        match instr {
            Instruction::Addx(_) => {
                for cycle_add in 0..2 {
                    let pixel_id = (i + cycle_add) % 40;
                    if ((registry - 1)..=(registry + 1)).contains(&pixel_id) {
                        crt_pixels.push_str("#");
                    } else {
                        crt_pixels.push_str(".");
                    }
                }
            }
            _ => {
                for cycle_add in 0..1 {
                    let pixel_id = (i + cycle_add) % 40;
                    if ((registry - 1)..=(registry + 1)).contains(&pixel_id) {
                        crt_pixels.push_str("#");
                    } else {
                        crt_pixels.push_str(".");
                    }
                }
            }
        };

        match instr {
            Instruction::Addx(q) => {
                registry += q;
                i += 2
            }
            _ => i += 1,
        };
    }
    crt_pixels
        .chars()
        .chunks(40)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .join("\n")
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "13140");
    }
    const RESULT2: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, RESULT2);
    }
}
