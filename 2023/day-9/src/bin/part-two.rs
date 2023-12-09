fn main() {
    let input: &str = include_str!("../../input.txt");
    if input.is_empty() {
        panic!("Input is empty");
    }
    println!("{}", process(input));
}

fn check_all_zero(data: &Vec<i64>) -> bool {
    data.iter().filter(|x| **x == 0).count() == data.len()
}
fn process(input: &str) -> i64 {
    let numbers = parse(input);

    numbers
        .into_iter()
        .map(|row| {
            let mut intermediate = Vec::new();
            let mut last_row = row.clone();
            while !check_all_zero(&last_row) {
                intermediate.push(last_row.clone());
                let mut previous = last_row[0];
                last_row = last_row
                    .into_iter()
                    .skip(1)
                    .map(|n| {
                        let res = n - previous;
                        previous = n;
                        res
                    })
                    .collect::<Vec<_>>();
            }

            let mut acc = 0;
            intermediate
                .into_iter()
                .map(|x| x[0])
                .rev()
                .map(|x| {
                    let res = x - acc;
                    acc = res;
                    res
                })
                .last()
                .unwrap()
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
        assert_eq!(process(INPUT_TEXT), 2);
    }
}
