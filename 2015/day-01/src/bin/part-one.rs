fn main() {
    let input: &str = include_str!("../../input.txt");
    if input.is_empty() {
        panic!("Input is empty");
    }
    println!("{}", process(input));
}

fn process(input: &str) -> isize {
    input.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEXT: &str = ")())())";

    #[test]
    fn test_part_one() {
        assert_eq!(process(INPUT_TEXT), -3);
    }
}
