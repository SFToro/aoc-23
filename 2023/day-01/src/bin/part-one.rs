fn main() {
    let input: &str = include_str!("../../part-one.txt");
    println!("{}", process(input));
}

fn process(input: &str) -> u32 {
    let result = input
        .lines()
        .map(|line| {
            let mut only_nums = line.chars().filter(|c| c.is_numeric());

            let first_digit = only_nums.next().unwrap().to_digit(10).unwrap();

            if let Some(second_digit) = only_nums.last() {
                first_digit * 10 + second_digit.to_digit(10).unwrap()
            } else {
                first_digit * 10 + first_digit
            }
        })
        .sum();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_part_one() {
        assert_eq!(process(INPUT_TEXT), 142);
    }
}
