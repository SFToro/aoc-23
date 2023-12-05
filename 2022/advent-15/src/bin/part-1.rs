use advent_15::process_part_1;
use color_eyre::eyre;
fn main() -> eyre::Result<()> {
    let input = include_str!("../../input.txt");
    println!("Answer: {}", process_part_1(input));
    Ok(())
}
