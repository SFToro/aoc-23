use std::fs;

fn main() {
    let binding = fs::read_to_string("input.txt").expect("File not found");
    let lines = binding.lines();
    let points: usize = lines
        .map(|line| {
            let mut my_line_iterator = line.split_whitespace();
            let movement_one = my_line_iterator.next().unwrap();
            let sign_two = my_line_iterator.next().unwrap();
            let movement_two = create_movement(movement_one, sign_two);
            check_points(movement_one, &movement_two)
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

// X  lose
// Y  draw
// Z  win

// A Rock
// B Paper
// C Scissors.

// X Rock
// Y Paper
// Z Scissors.
fn create_movement(movement_one: &str, sign_two: &str) -> String {
    let movement_two: String;
    match movement_one {
        "A" => match sign_two {
            "X" => movement_two = String::from("Z"),
            "Y" => movement_two = String::from("X"),
            "Z" => movement_two = String::from("Y"),
            _ => movement_two = String::from(""),
        },
        "B" => match sign_two {
            "X" => movement_two = String::from("X"),
            "Y" => movement_two = String::from("Y"),
            "Z" => movement_two = String::from("Z"),
            _ => movement_two = String::from(""),
        },
        "C" => match sign_two {
            "X" => movement_two = String::from("Y"),
            "Y" => movement_two = String::from("Z"),
            "Z" => movement_two = String::from("X"),
            _ => movement_two = String::from(""),
        },
        _ => movement_two = String::from(""),
    }
    movement_two
}
