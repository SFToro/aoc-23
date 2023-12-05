use std::cmp::Ordering;

use nom::{
    branch::alt, bytes::complete::*, character::complete::newline, multi::separated_list1,
    sequence::preceded, IResult, Parser,
};

#[derive(Debug, Clone, Copy)]
enum Op {
    Multiplication(i32),
    Sum(i32),
    Sqr,
}
#[derive(Debug)]
struct Monkey {
    id: i32,
    items: Vec<u128>,
    op: Op,
    test: i32,
    true_dir: i32,
    false_dir: i32,
    inspected: i32,
}
impl Ord for Monkey {
    fn cmp(&self, other: &Self) -> Ordering {
        other.inspected.cmp(&self.inspected)
    }
}

impl PartialOrd for Monkey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Monkey {
    fn eq(&self, other: &Self) -> bool {
        self.inspected == other.inspected
    }
}
impl Eq for Monkey {}
fn parse_items(input: &str) -> IResult<&str, Vec<u128>> {
    let (input, numbers) = preceded(
        tag("  Starting items: "),
        separated_list1(tag(", "), nom::character::complete::i32),
    )(input)?;
    let numbers = numbers.into_iter().map(|n| n as u128).collect();
    Ok((input, numbers))
}
fn parse_operation(input: &str) -> IResult<&str, Op> {
    let sum = preceded(tag("+ "), nom::character::complete::i32);
    let sq = tag("* old");
    let mult = preceded(tag("* "), nom::character::complete::i32);
    let (input, _) = tag("  Operation: new = old ")(input)?;

    alt((
        sum.map(Op::Sum),
        sq.map(|_| Op::Sqr),
        mult.map(Op::Multiplication),
    ))(input)
}
fn parse_test(input: &str) -> IResult<&str, i32> {
    let (input, number) =
        preceded(tag("  Test: divisible by "), nom::character::complete::i32)(input)?;
    Ok((input, number))
}
fn parse_directions(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, true_dir) = preceded(
        tag("    If true: throw to monkey "),
        nom::character::complete::i32,
    )(input)?;
    let (input, _) = newline(input)?;
    let (input, false_dir) = preceded(
        tag("    If false: throw to monkey "),
        nom::character::complete::i32,
    )(input)?;
    Ok((input, (true_dir, false_dir)))
}
fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, id) = preceded(tag("Monkey "), nom::character::complete::i32)(input)?;
    let (input, _) = tag(":\n")(input)?;
    let (input, items) = parse_items(input)?;
    let (input, _) = newline(input)?;
    let (input, op) = parse_operation(input)?;
    let (input, _) = newline(input)?;
    let (input, test) = parse_test(input)?;
    let (input, _) = newline(input)?;
    let (input, (true_dir, false_dir)) = parse_directions(input)?;

    Ok((
        input,
        Monkey {
            id,
            items,
            op,
            test,
            true_dir,
            false_dir,
            inspected: 0,
        },
    ))
}
fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    let (input, monkeys) = separated_list1(tag("\n\n"), parse_monkey)(input)?;
    Ok((input, monkeys))
}

pub fn process_part1(input: &str) -> String {
    let rounds = 20;
    let (_input, mut monkeys) = parse_monkeys(input).unwrap();
    dbg!(&monkeys);
    for i in 0..rounds {
        for j in 0..monkeys.len() {
            if !monkeys[j].items.is_empty() {
                for i in 0..monkeys[j].items.len() {
                    monkeys[j].inspected += 1;
                    let mut new_item = match monkeys[j].op {
                        Op::Multiplication(x) => monkeys[j].items[i] * (x as u128),
                        Op::Sum(x) => monkeys[j].items[i] + x as u128,
                        Op::Sqr => monkeys[j].items[i] * monkeys[j].items[i],
                    };
                    new_item /= 3;
                    dbg!(&(new_item));
                    match (new_item % monkeys[j].test as u128) == 0 {
                        true => {
                            let dir = &(monkeys[j].true_dir as usize);
                            monkeys[*dir].items.push(new_item)
                        }
                        false => {
                            let dir = &(monkeys[j].false_dir as usize);
                            monkeys[*dir].items.push(new_item)
                        }
                    }
                }
                monkeys[j].items = vec![];
            }
        }
    }
    monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));
    (monkeys[0].inspected * monkeys[1].inspected).to_string()
}

pub fn process_part2(input: &str) -> String {
    let rounds = 10_000;
    let (_input, mut monkeys) = parse_monkeys(input).unwrap();
    let magic_trick = monkeys
        .iter()
        .map(|monkey| monkey.test as u128)
        .product::<u128>();
    for _ in 0..rounds {
        for j in 0..monkeys.len() {
            if !monkeys[j].items.is_empty() {
                for i in 0..monkeys[j].items.len() {
                    monkeys[j].inspected += 1;
                    let mut new_item = match monkeys[j].op {
                        Op::Multiplication(x) => monkeys[j].items[i] * x as u128,
                        Op::Sum(x) => monkeys[j].items[i] + x as u128,
                        Op::Sqr => monkeys[j].items[i] * monkeys[j].items[i],
                    };
                    new_item %= magic_trick;
                    match (new_item % monkeys[j].test as u128) == 0 {
                        true => {
                            let dir = &(monkeys[j].true_dir as usize);
                            monkeys[*dir].items.push(new_item)
                        }
                        false => {
                            let dir = &(monkeys[j].false_dir as usize);
                            monkeys[*dir].items.push(new_item)
                        }
                    }
                }
                monkeys[j].items = vec![];
            }
        }
    }
    // monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));
    monkeys.sort();
    dbg!(&monkeys);
    (monkeys[0].inspected as u128 * monkeys[1].inspected as u128).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
    #[test]
    fn part1_works() {
        let _result = process_part1(INPUT);
        assert_eq!(_result, "10605");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "2713310158");
    }
}
