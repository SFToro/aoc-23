use advent_12::process_part_2;
use color_eyre::eyre;
fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let input = include_bytes!("../../input.txt");
    println!("Answer: {}", process_part_2(input.to_vec()));
    Ok(())
}
