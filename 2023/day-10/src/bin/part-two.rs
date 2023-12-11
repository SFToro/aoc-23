use core::panic;
use std::{collections::BTreeMap, fs::File, io::Write};

use itertools::Itertools;

fn main() {
    let input: &str = include_str!("../../input.txt");
    if input.is_empty() {
        panic!("Input is empty");
    }
    println!("{}", process(input));
}

const TO_CHECK: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

fn process(input: &str) -> isize {
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

    let mut visited = Vec::new();
    visited.push(*S);

    while current_pos != *S {
        loops += 1;
        visited.push(current_pos);
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
    visited.push(*S);

    let mut sum = 0;
    for (idx, (x, y)) in visited.iter().enumerate().take(visited.len() - 1) {
        sum += (x * visited[idx + 1].1) - (visited[idx + 1].0 * y)
    }
    sum = sum.abs() / 2;

    let perimeter = visited.len() as isize - 1;
    dbg!(perimeter);

    (sum - ((perimeter) / 2)) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
    const INPUT_TEXT2: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    const INPUT_TEXT3: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    #[test]
    fn test_part_two() {
        assert_eq!(process(INPUT_TEXT), 10);
        assert_eq!(process(INPUT_TEXT2), 8);
        assert_eq!(process(INPUT_TEXT3), 4);
    }
}
