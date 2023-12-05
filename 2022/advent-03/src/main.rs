use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").expect("File not found");
    let lines = input.lines();
    let sum: usize = lines
        .map(|line| {
            let x = &line[0..line.len() / 2];
            let y = &line[line.len() / 2..];
            let premio = check_repeated(x, y);
            check_premio(premio)
        })
        .sum();
    println!("{:?}", sum);
}

fn check_repeated(left_compartiment: &str, right_compartiment: &str) -> char {
    let my_match: Vec<_> = left_compartiment
        .chars()
        .filter(|l| right_compartiment.contains(*l))
        .collect();
    my_match[0]
}

fn check_premio(character: char) -> usize {
    let ascii = character as usize;
    if ascii > 96 {
        return ascii - 96;
    } else {
        return ascii - 38;
    }
}
