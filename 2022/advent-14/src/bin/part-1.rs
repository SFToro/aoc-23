use advent_13::process_part_1;
// use anyhow::{Context, Result};
use color_eyre::eyre::{Context, Result};
fn main() -> Result<()> {
    // fn main() -> color_eyre::eyre::Result<()> {
    // color_eyre::install()?;
    let input = include_str!("../../input.txt");
    println!("Answer: {}", process_part_1(input).unwrap());
    Ok(())
}
