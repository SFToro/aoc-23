fn main() {
    let input: &str = include_str!("../../input.txt");
    if input.is_empty() {
        panic!("Input is empty");
    }
    println!("{}", process(input));
}

fn process(input: &str) -> i64 {
    parse(input)
        .into_iter()
        .map(|mut row| {
            let mut sum_of_developed_lasts = *row.last().unwrap();
            while !row.iter().all(|x| *x == 0) {
                row = row.windows(2).map(|r| r[1] - r[0]).collect();
                sum_of_developed_lasts += *row.last().unwrap();
            }
            sum_of_developed_lasts
        })
        .sum()
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.parse().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part_one() {
        assert_eq!(process(INPUT_TEXT), 114);
    }
}
