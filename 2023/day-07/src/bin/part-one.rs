use std::collections::BTreeMap;

use nom::{
    character::complete::{alphanumeric1, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
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
    let (_, mut rounds) = parse_input(input).unwrap();
    rounds.sort();
    rounds
        .iter()
        .enumerate()
        .map(|(index, round)| round.bid * ((index as u64) + 1))
        .sum()
}

const CARDS: [(char, u64); 13] = [
    ('2', 2),
    ('3', 3),
    ('4', 4),
    ('5', 5),
    ('6', 6),
    ('7', 7),
    ('8', 8),
    ('9', 9),
    ('T', 10),
    ('J', 11),
    ('Q', 12),
    ('K', 13),
    ('A', 14),
];
#[derive(Clone, Copy)]
enum Hand {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

impl Hand {}

struct Round<'a> {
    hand: Hand,
    cards: &'a str,
    bid: u64,
}
impl Round<'_> {
    fn classify_hand(input: &str) -> Hand {
        let mut counts = BTreeMap::new();
        for ch in input.chars() {
            *counts.entry(ch).or_insert(0) += 1;
        }

        let mut counts = counts.into_iter().collect::<Vec<_>>();
        counts.sort_by_key(|(_, count)| *count);
        let (_ch, count) = counts.pop().unwrap();
        match count {
            5 => Hand::FiveOfAKind,
            4 => Hand::FourOfAKind,
            3 => {
                let (_ch2, count2) = counts.pop().unwrap();
                match count2 {
                    2 => Hand::FullHouse,
                    1 => Hand::ThreeOfAKind,
                    _ => panic!("3 same cards but no full house or three of a kind"),
                }
            }
            2 => {
                let (_ch2, count2) = counts.pop().unwrap();
                match count2 {
                    2 => Hand::TwoPair,
                    1 => Hand::OnePair,
                    _ => panic!("2 same cards but no two pair or one pair"),
                }
            }
            1 => Hand::HighCard,
            _ => panic!("Invalid hand"),
        }
    }
}
impl Eq for Round<'_> {}

impl PartialEq for Round<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for Round<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Round<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if (self.hand as u8) == (other.hand as u8) {
            return {
                let answer = &self
                    .cards
                    .chars()
                    .zip(other.cards.chars())
                    .find_map(|(card, other_card)| {
                        let card_power = CARDS.iter().find(|(ch, _power)| *ch == card).unwrap().1;
                        let other_card_power = CARDS
                            .iter()
                            .find(|(ch, _power)| *ch == other_card)
                            .unwrap()
                            .1;
                        if card_power != other_card_power {
                            Some(card_power.cmp(&other_card_power))
                        } else {
                            None
                        }
                    })
                    .unwrap_or(std::cmp::Ordering::Equal);
                *answer
            };
        } else {
            (self.hand as u8).cmp(&(other.hand as u8))
        }
    }
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    let (input, (hand, bid)) =
        separated_pair(alphanumeric1, space1, nom::character::complete::u64)(input)?;

    let hand_variant = Round::classify_hand(hand);

    Ok((
        input,
        Round {
            hand: hand_variant,
            bid,
            cards: hand,
        },
    ))
}
fn parse_input(input: &str) -> IResult<&str, Vec<Round>> {
    let (input, rounds) = separated_list1(line_ending, parse_round)(input)?;
    Ok((input, rounds))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part_one() {
        assert_eq!(process(INPUT_TEXT), 6440);
    }
}
