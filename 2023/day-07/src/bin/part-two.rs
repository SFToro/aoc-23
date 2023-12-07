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
    ('J', 1),
    ('2', 2),
    ('3', 3),
    ('4', 4),
    ('5', 5),
    ('6', 6),
    ('7', 7),
    ('8', 8),
    ('9', 9),
    ('T', 10),
    ('Q', 11),
    ('K', 12),
    ('A', 13),
];

#[derive(Debug)]
enum Hand<'a> {
    HighCard(&'a str),
    OnePair(&'a str),
    TwoPair(&'a str),
    ThreeOfAKind(&'a str),
    FullHouse(&'a str),
    FourOfAKind(&'a str),
    FiveOfAKind(&'a str),
}

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Hand::FiveOfAKind(_) => match other {
                Hand::FiveOfAKind(_) => self.compare_against_same_hand(&other),
                _ => std::cmp::Ordering::Greater,
            },
            Hand::FourOfAKind(_) => match other {
                Hand::FiveOfAKind(_) => std::cmp::Ordering::Less,
                Hand::FourOfAKind(_) => self.compare_against_same_hand(&other),

                _ => std::cmp::Ordering::Greater,
            },
            Hand::FullHouse(_) => match other {
                Hand::FiveOfAKind(_) => std::cmp::Ordering::Less,
                Hand::FourOfAKind(_) => std::cmp::Ordering::Less,
                Hand::FullHouse(_) => self.compare_against_same_hand(&other),
                _ => std::cmp::Ordering::Greater,
            },
            Hand::ThreeOfAKind(_) => match other {
                Hand::FiveOfAKind(_) => std::cmp::Ordering::Less,
                Hand::FourOfAKind(_) => std::cmp::Ordering::Less,
                Hand::FullHouse(_) => std::cmp::Ordering::Less,
                Hand::ThreeOfAKind(_) => self.compare_against_same_hand(&other),
                _ => std::cmp::Ordering::Greater,
            },
            Hand::TwoPair(_) => match other {
                Hand::TwoPair(_) => self.compare_against_same_hand(&other),
                Hand::OnePair(_) => std::cmp::Ordering::Greater,
                Hand::HighCard(_) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Less,
            },
            Hand::OnePair(_) => match other {
                Hand::OnePair(_) => self.compare_against_same_hand(&other),
                Hand::HighCard(_) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Less,
            },
            Hand::HighCard(_) => match other {
                Hand::HighCard(_) => self.compare_against_same_hand(&other),
                _ => std::cmp::Ordering::Less,
            },
        }
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Hand<'_> {}

impl PartialEq for Hand<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.get_string() == other.get_string()
    }
}

impl Hand<'_> {
    fn compare_against_same_hand(&self, other: &Self) -> std::cmp::Ordering {
        let answer = &self
            .get_string()
            .chars()
            .zip(other.get_string().chars())
            .find_map(|(card, other_card)| {
                let card_power = Hand::get_power_of_card(card);
                let other_card_power = Hand::get_power_of_card(other_card);
                if card_power != other_card_power {
                    Some(card_power.cmp(&other_card_power))
                } else {
                    None
                }
            })
            .unwrap_or(std::cmp::Ordering::Equal);
        *answer
    }
    fn get_string(&self) -> &str {
        match self {
            Hand::HighCard(s) => s,
            Hand::OnePair(s) => s,
            Hand::TwoPair(s) => s,
            Hand::ThreeOfAKind(s) => s,
            Hand::FullHouse(s) => s,
            Hand::FourOfAKind(s) => s,
            Hand::FiveOfAKind(s) => s,
        }
    }
    fn get_power_of_card(c: char) -> usize {
        CARDS
            .iter()
            .position(|(ch, _)| *ch == c)
            .expect("Invalid position in the const CARDS array")
    }
    fn classify(input: &str) -> Hand {
        let mut counts = BTreeMap::new();
        for ch in input.chars() {
            *counts.entry(ch).or_insert(0) += 1;
        }
        let mut counts = counts.into_iter().collect::<Vec<_>>();
        counts.sort_by(|(ch1, _count1), (ch2, _count2)| {
            let card_power1 = Hand::get_power_of_card(*ch1);
            let card_power2 = Hand::get_power_of_card(*ch2);
            card_power1.cmp(&card_power2)
        });
        counts.sort_by_key(|(_, count)| *count);

        let jokers = counts
            .iter()
            .find(|(ch, _count)| ch == &'J')
            .unwrap_or(&('J', 0))
            .1;
        let last = counts.pop().unwrap();
        let _flag_last = last.1;

        let before_last = counts.pop().unwrap_or(('J', 0));

        if last.0 == 'J' {
            counts.push((last.0, last.1 + before_last.1))
        } else {
            counts.push(before_last);
            counts.push((last.0, last.1 + jokers));
        }

        let (_ch, count) = counts.pop().unwrap();
        match count {
            5 => Hand::FiveOfAKind(input),
            4 => Hand::FourOfAKind(input),
            3 => {
                let (_ch2, count2) = counts.pop().unwrap();
                match count2 {
                    2 => Hand::FullHouse(input),
                    1 => Hand::ThreeOfAKind(input),
                    _ => panic!("3 same cards but no full house or three of a kind"),
                }
            }
            2 => {
                let (_ch2, count2) = counts.pop().unwrap();
                match count2 {
                    2 => Hand::TwoPair(input),
                    1 => Hand::OnePair(input),
                    _ => panic!("2 same cards but no two pair or one pair"),
                }
            }
            1 => Hand::HighCard(input),
            _ => panic!("Invalid hand {}", input),
        }
    }
}
#[derive(Debug)]
struct Round<'a> {
    hand: Hand<'a>,
    bid: u64,
}

impl Eq for Round<'_> {}

impl PartialEq for Round<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl PartialOrd for Round<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Round<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand.cmp(&other.hand)
    }
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    let (input, (hand, bid)) =
        separated_pair(alphanumeric1, space1, nom::character::complete::u64)(input)?;

    let hand = Hand::classify(hand);

    Ok((input, Round { hand, bid }))
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
        assert_eq!(process(INPUT_TEXT), 5905);
    }
}
