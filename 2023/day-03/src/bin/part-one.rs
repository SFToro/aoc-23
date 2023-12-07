use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let input: &str = include_str!("../../input.txt");
    println!("{}", process(input));
}

struct Last {
    chars: Vec<char>,
    starting_index: i32,
}
impl Last {
    fn reset(&mut self) {
        self.chars = vec![];
        self.starting_index = -1;
    }
}

fn process(input: &str) -> usize {
    let lines = input.lines();
    let mut grid: BTreeMap<(usize, usize), String> = BTreeMap::new();
    let mut last = Last {
        chars: vec![],
        starting_index: -1,
    };
    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                if last.starting_index == -1 {
                    last.starting_index = j as i32;
                }
                last.chars.push(c);
            } else {
                if last.starting_index != -1 {
                    grid.insert(
                        (i, last.starting_index as usize),
                        last.chars.iter().collect(),
                    );
                }
                if c != '.' {
                    grid.insert((i, j), c.into());
                }
                last.reset();
            }
        }
        if last.starting_index != -1 {
            grid.insert(
                (i, last.starting_index as usize),
                last.chars.iter().collect(),
            );
        }
        last.reset();
    }

    let mut required = BTreeSet::new();
    'outer: for (k, v) in grid.iter() {
        let mut adjacents: Vec<(i32, i32)> = vec![];
        if v.parse::<usize>().is_ok() {
            let len = v.len();
            adjacents.push((k.0 as i32 - 1, k.1 as i32 - 1));
            adjacents.push((k.0 as i32, k.1 as i32 - 1));
            adjacents.push((k.0 as i32 + 1, k.1 as i32 - 1));

            adjacents.push((k.0 as i32 - 1, k.1 as i32 + len as i32));
            adjacents.push((k.0 as i32, k.1 as i32 + len as i32));
            adjacents.push((k.0 as i32 + 1, k.1 as i32 + len as i32));

            for i in 0..len {
                adjacents.push((k.0 as i32 - 1, k.1 as i32 + i as i32));

                adjacents.push((k.0 as i32 + 1, k.1 as i32 + i as i32));
            }

            for adj in adjacents {
                let (x, y) = (adj.0 as usize, adj.1 as usize);
                if grid.contains_key(&(x, y)) {
                    let adjascent = grid.get(&(x, y)).unwrap();
                    if adjascent.parse::<usize>().is_err() {
                        required.insert(k);
                        continue 'outer;
                    }
                }
            }
        }
    }

    required
        .into_iter()
        .map(|(x, y)| {
            if grid.contains_key(&(*x, *y)) {
                grid.get(&(*x, *y)).unwrap().parse::<usize>().unwrap()
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part_one() {
        assert_eq!(process(INPUT_TEXT), 4361);
    }
}
