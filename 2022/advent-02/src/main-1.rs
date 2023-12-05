use std::fs;

fn main() {
    let binding = fs::read_to_string("input.txt").expect("File not found");
    let lines = binding.lines();
    let points: usize = lines
        .map(|line| {
            let mut my_line_iterator = line.split_whitespace();
            let movement_one = my_line_iterator.next().unwrap();
            let movement_two = my_line_iterator.next().unwrap();
            check_points(movement_one, movement_two)
        })
        .sum();

    println!("{:?}", points);
}

// A Rock
// B Paper
// C Scissors.

// X Rock
// Y Paper
// Z Scissors.
fn check_points(movement_one: &str, movement_two: &str) -> usize {
    let mut points = 0;
    match movement_one {
        "A" => match movement_two {
            "X" => points += 1 + 3,
            "Y" => points += 2 + 6,
            "Z" => points += 3,
            _ => points += 0,
        },
        "B" => match movement_two {
            "X" => points += 1,
            "Y" => points += 2 + 3,
            "Z" => points += 3 + 6,
            _ => points += 0,
        },
        "C" => match movement_two {
            "X" => points += 1 + 6,
            "Y" => points += 2,
            "Z" => points += 3 + 3,
            _ => points += 0,
        },
        _ => points += 0,
    }
    points
}
