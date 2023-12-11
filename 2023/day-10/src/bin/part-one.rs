use core::panic;
use std::collections::BTreeMap;

fn main() {
    let input: &str = include_str!("../../input.txt");
    if input.is_empty() {
        panic!("Input is empty");
    }
    println!("{}", process(input));
}

const TO_CHECK: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

fn process(input: &str) -> usize {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| ((i as isize, j as isize), c))
                .collect::<Vec<_>>()
        })
        .collect::<BTreeMap<_, _>>();

    let (S, _) = map
        .iter()
        .find(|(k, v)| **v == 'S')
        .expect("Starting point not found but S should exist");

    let starting_positions: Vec<_> = TO_CHECK
        .iter()
        .filter_map(|(x, y)| {
            let node_to_check = (S.0 + x, S.1 + y);
            match (x, y) {
                (1, 0) => match map.get(&node_to_check) {
                    Some('|') => Some((node_to_check)),
                    Some('L') => Some((node_to_check)),
                    Some('J') => Some((node_to_check)),
                    _ => None,
                },
                (0, 1) => match map.get(&node_to_check) {
                    Some('-') => Some((node_to_check)),
                    Some('J') => Some((node_to_check)),
                    Some('7') => Some((node_to_check)),
                    _ => None,
                },
                (-1, 0) => match map.get(&node_to_check) {
                    Some('|') => Some((node_to_check)),
                    Some('F') => Some((node_to_check)),
                    Some('7') => Some((node_to_check)),
                    _ => None,
                },
                (0, -1) => match map.get(&node_to_check) {
                    Some('-') => Some((node_to_check)),
                    Some('L') => Some((node_to_check)),
                    Some('F') => Some((node_to_check)),
                    _ => None,
                },
                _ => unreachable!("Unreachable"),
            }
        })
        .collect();

    let mut loops = 0;
    let mut current_pos = starting_positions[0];
    let mut previous = *S;

    while current_pos != *S {
        loops += 1;

        let mut next: Vec<_> = TO_CHECK
            .iter()
            .filter_map(|(x, y)| {
                let current_char = map
                    .get(&current_pos)
                    .expect("current char comes from a validated position");

                let allowed_by_current_char = match current_char {
                    '-' => [(0, -1), (0, 1), (0, 0), (0, 0)],
                    'L' => [(-1, 0), (0, 1), (0, 0), (0, 0)],
                    'F' => [(1, 0), (0, 1), (0, 0), (0, 0)],
                    '|' => [(1, 0), (-1, 0), (0, 0), (0, 0)],
                    'J' => [(0, -1), (-1, 0), (0, 0), (0, 0)],
                    '7' => [(0, -1), (1, -0), (0, 0), (0, 0)],
                    value => unreachable!("Unrecognized symbol {}", value),
                };

                if let Some(pair) = allowed_by_current_char
                    .iter()
                    .cloned()
                    .find(|(m, n)| m == x && n == y)
                {
                    match map.get(&(&pair.0 + current_pos.0, &pair.1 + current_pos.1)) {
                        Some('.') => None,
                        Some('S') => None,
                        Some(_) => {
                            let p = previous.clone();
                            if (p.0 == &pair.0 + current_pos.0 && p.1 == &pair.1 + current_pos.1) {
                                None
                            } else {
                                Some((&pair.0 + current_pos.0, &pair.1 + current_pos.1))
                            }
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .collect();
        previous = current_pos;
        current_pos = match next.into_iter().next() {
            Some(pos) => pos,
            None => break,
        }
    }
    loops / 2 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT: &str = ".....
.S-7.
.|.|.
.L-J.
.....";
    const INPUT_TEXT2: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    #[test]
    fn test_part_one() {
        assert_eq!(process(INPUT_TEXT), 4);
        assert_eq!(process(INPUT_TEXT2), 8);
    }
}
