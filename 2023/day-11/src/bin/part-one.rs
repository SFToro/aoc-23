use day_11::part_one::expand;

use std::collections::BTreeMap;
fn main() {
    let input: &str = include_str!("../../input.txt");
    if input.is_empty() {
        panic!("Input is empty");
    }
    println!("{}", process(input));
}

fn process(input: &str) -> usize {
    let new_map = expand(input);

    let new_map: BTreeMap<_, _> = new_map
        .into_iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.into_iter()
                .enumerate()
                .map(|(j, v)| ((i, j), v))
                .collect::<Vec<_>>()
        })
        .collect();
    let galaxies: Vec<_> = new_map.iter().filter(|((_i, _j), v)| **v == '#').collect();

    let mut distances = Vec::new();
    for (idx, ((i, j), _galaxy)) in galaxies.iter().enumerate() {
        for ((x, y), _gal) in galaxies.iter().skip(idx + 1) {
            let x = *x as isize;
            let y = *y as isize;
            let i = *i as isize;
            let j = *j as isize;
            // distances.push(((i, j), (x, y), (i - x).abs() + (j - y).abs()))
            distances.push(((i - x).abs() + (j - y).abs()) as usize)
        }
    }
    distances.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use day_11::part_one::print;

    const INPUT_TEXT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_expand() {
        let expanded_data = expand(INPUT_TEXT);
        let result = print(expanded_data);

        assert_eq!(
            result,
            "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......
"
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(process(INPUT_TEXT), 374);
    }
}
