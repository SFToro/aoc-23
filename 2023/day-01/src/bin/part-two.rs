use std::cmp::Ordering;

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const VALUES: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

fn main() {
    let input: &str = include_str!("../../part-two.txt");
    println!("{}", process(input));
}

struct Number {
    value: u8,
    pos: usize,
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Number {}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pos.cmp(&other.pos)
    }
}

fn process(input: &str) -> usize {
    let result: usize = input
        .lines()
        .map(|line| {
            let mut vec: Vec<Number> = Vec::new();
            for (i, number) in NUMBERS.iter().enumerate() {
                if line.contains(number) {
                    for find in line.match_indices(number) {
                        vec.push(Number {
                            value: VALUES[i],
                            pos: find.0,
                        });
                    }
                }
            }
            line.chars()
                .enumerate()
                .filter(|c| (c.1).is_numeric())
                .for_each(|c| {
                    vec.push(Number {
                        value: (c.1).to_digit(10).unwrap() as u8,
                        pos: c.0,
                    });
                });
            vec.sort();

            let number =
                (vec.first().unwrap().value * 10) as usize + vec.last().unwrap().value as usize;

            number
        })
        .sum();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_part_two() {
        assert_eq!(process(INPUT_TEXT), 281);
    }
}
