use day_11::part_two::*;

fn main() {
    let input: &str = include_str!("../../input.txt");
    if input.is_empty() {
        panic!("Input is empty");
    }
    println!("{}", process(input, 1_000_000));
}

fn process(input: &str, times: usize) -> usize {
    let (map, rows, columns) = parse(input);

    let galaxies: Vec<_> = map.iter().filter(|((_i, _j), v)| **v == '#').collect();

    galaxies
        .iter()
        .enumerate()
        .map(|(idx, ((i, j), _galaxy))| {
            galaxies
                .iter()
                .skip(idx + 1)
                .map(|((x, y), _gal)| {
                    let extra_distance = columns
                        .iter()
                        .filter_map(|c| (*j > *c && y < c || *j < *c && y > c).then_some(times - 1))
                        .sum::<usize>()
                        + rows
                            .iter()
                            .filter_map(|c| {
                                (*i > *c && x < c || *i < *c && x > c).then_some(times - 1)
                            })
                            .sum::<usize>();

                    let x = *x as isize;
                    let y = *y as isize;
                    let i = *i as isize;
                    let j = *j as isize;

                    ((i - x).abs() + (j - y).abs()) as usize + extra_distance
                })
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_part_two() {
        assert_eq!(process(INPUT_TEXT, 10), 1030);
        assert_eq!(process(INPUT_TEXT, 100), 8410);
    }
}
