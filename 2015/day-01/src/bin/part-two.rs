fn main() {
    let input: &str = include_str!("../../input.txt");
    if input.is_empty() {
        panic!("Input is empty");
    }
    println!("{}", process(input));
}

fn process(input: &str) -> isize {
    let mut floor = 0;
    input
        .chars()
        .enumerate()
        .find_map(|(idx, c)| {
            if floor == -1 {
                Some(idx as isize)
            } else {
                floor += match c {
                    '(' => 1,
                    ')' => -1,
                    _ => 0,
                };
                None
            }
        })
        .expect("Never arrived to the basement")
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
