use std::{cell::Cell, collections::BTreeMap};

use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric0, line_ending},
    multi::{many0, separated_list0, separated_list1},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
struct Game {
    card_nr: usize,
    winners: Vec<usize>,
    my_numbers: Vec<usize>,
}
#[derive(Debug, Clone)]
struct GameResult {
    card_nr: usize,
    ids: Vec<usize>,
    amount: Cell<usize>,
}

fn main() {
    let input: &str = include_str!("../../input.txt");
    println!("{}", process(input));
}

fn process(input: &str) -> usize {
    let (_, games) = parse_input(input).unwrap();

    let game_results = games
        .iter()
        .map(|game| {
            let results = game
                .my_numbers
                .iter()
                .filter(|my_number| game.winners.contains(*my_number))
                .collect::<Vec<_>>();
            let ids = (1..=results.len()).map(|x| x + game.card_nr).collect();
            GameResult {
                card_nr: game.card_nr,
                ids,
                amount: Cell::new(1),
            }
        })
        .collect::<Vec<_>>();

    let btree_map: BTreeMap<usize, GameResult> = game_results
        .into_iter()
        .map(|result| (result.card_nr, result))
        .collect();

    for value in btree_map.values() {
        for id in &value.ids {
            let amount_to_update = btree_map.get(id).unwrap();

            amount_to_update
                .amount
                .set(amount_to_update.amount.get() + value.amount.get());
        }
    }

    btree_map.values().map(|v| v.amount.get()).sum()
}

fn parse_numbers(input: &str) -> nom::IResult<&str, Vec<usize>> {
    let (input, numbers) = separated_list0(tag(" "), alphanumeric0)(input)?;
    let numbers = numbers
        .into_iter()
        .filter_map(|number| number.parse::<usize>().ok())
        .collect();

    Ok((input, numbers))
}

fn parse_line(input: &str) -> nom::IResult<&str, Game> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = many0(tag(" "))(input)?;

    let (input, number) = alphanumeric0(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, (winners, my_numbers)) =
        separated_pair(parse_numbers, tag("|"), parse_numbers)(input)?;

    Ok((
        input,
        Game {
            card_nr: number.parse().unwrap(),
            my_numbers,
            winners,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, result) = separated_list1(line_ending, parse_line)(input)?;

    Ok((input, result))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part_one() {
        assert_eq!(process(INPUT_TEXT), 30);
    }
}
