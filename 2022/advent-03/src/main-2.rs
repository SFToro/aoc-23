use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").expect("File not found");
    let binding = input.split("\n").collect::<Vec<_>>();
    let lines = binding.chunks(3);
    let suma = lines
        .map(|three_lines| {
            let letra = check_repeated(three_lines[0], three_lines[1], three_lines[2]);
            match letra {
                Some(a) => {
                    println!("{}", a);
                    check_premio(a)
                }
                _ => 0,
            }
        })
        .sum::<usize>();
    println!("{}", suma)
}

fn check_repeated(one: &str, two: &str, three: &str) -> Option<char> {
    for letter in one.chars() {
        for letra in two.chars() {
            if letter == letra {
                for letre in three.chars() {
                    if letre == letra {
                        return Option::Some(letra);
                    }
                }
            }
        }
    }
    return Option::None;
}

fn check_premio(character: char) -> usize {
    let ascii = character as usize;
    if ascii > 96 {
        return ascii - 96;
    } else {
        return ascii - 38;
    }
}
