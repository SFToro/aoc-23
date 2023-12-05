use std::collections::BTreeMap;

use nom::{
    bytes::complete::{tag, take_until, take_while1},
    character::{complete::alphanumeric0, is_digit},
    combinator::map_res,
    error::VerboseError,
    multi::separated_list0,
    IResult,
};

fn main() {
    let input: &str = include_str!("../../input.txt");

    println!("{}", process(input));
}

fn process(input: &str) -> usize {
    let map: BTreeMap<&str, usize> = BTreeMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let mut acc = 0;
    for line in input.lines() {
        let (_, record) = parse_line(line).unwrap();
        if record.is_valid(&map) {
            acc += record.game;
        };
    }

    acc
}

#[derive(Debug)]
struct GameRecord {
    game: usize,
    plays: Vec<Play>,
}

impl GameRecord {
    fn is_valid(&self, limits: &BTreeMap<&str, usize>) -> bool {
        self.plays.iter().all(|play| {
            play.iter().all(|movement| {
                if let Some(limit) = limits.get(movement.color.as_str()) {
                    *limit >= movement.number
                } else {
                    false
                }
            })
        })
    }
}
type Play = Vec<Movement>;

#[derive(Debug)]
struct Movement {
    number: usize,
    color: String,
}

fn parse_line(input: &str) -> IResult<&str, GameRecord> {
    let (items, input) = take_until(":")(input)?;
    let (items, _) = tag(": ")(items)?;
    let (_, plays) = separated_list0(
        tag("; "),
        map_res(take_while1(|c| c != ';'), |s: &str| {
            let (_, plays_per_subgame) = separated_list0(
                tag::<_, _, VerboseError<&str>>(", "),
                map_res(take_while1(|c| c != ','), |movement: &str| {
                    let (input, number) =
                        take_while1::<_, _, VerboseError<&str>>(|c: char| is_digit(c as u8))(
                            movement,
                        )?;

                    let (input, _) = tag::<_, _, VerboseError<&str>>(" ")(input)?;
                    let (_, color) = alphanumeric0::<_, VerboseError<&str>>(input)?;

                    Ok::<_, nom::Err<VerboseError<&str>>>(Movement {
                        number: number.parse::<usize>().unwrap(),
                        color: color.to_string(),
                    })
                }),
            )(s)
            .unwrap();

            Ok::<_, nom::Err<VerboseError<&str>>>(plays_per_subgame)
        }),
    )(items)?;

    let (game, _) = tag("Game ")(input)?;
    Ok((
        "",
        GameRecord {
            game: game.parse::<usize>().unwrap(),
            plays,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part_two() {
        assert_eq!(process(INPUT_TEXT), 8);
    }
}
