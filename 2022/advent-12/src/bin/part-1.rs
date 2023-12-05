use advent_12::process_part_1;
fn main() {
    let input = include_bytes!("../../input.txt");
    println!("Answer: {}", process_part_1(input.to_vec()))
}
