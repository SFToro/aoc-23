use advent_11::process_part1;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let contents = include_str!("../../input.txt");
    println!("{}", process_part1(contents));
    Ok(())
}
