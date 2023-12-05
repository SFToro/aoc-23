use std::fs;
fn main() {
    let content = fs::read_to_string("input.txt").expect("Cannot load the file");
    let lines = content.lines();
    let overlaps: usize = lines
        .filter(|line| {
            let (zone_one, zone_two) = line.split_once(",").unwrap();
            is_overlapped(zone_one, zone_two)
        })
        .count();
    println!("{:?}", overlaps);
}

fn is_overlapped(zone_one: &str, zone_two: &str) -> bool {
    let boundaries_one = zone_boundaries(zone_one);
    let boundaries_two = zone_boundaries(zone_two);

    if (boundaries_one[0]..boundaries_one[1] + 1).contains(&boundaries_two[0])
        || (boundaries_two[0]..boundaries_two[1] + 1).contains(&boundaries_one[0])
    {
        return true;
    }
    false
}
fn zone_boundaries(zone: &str) -> Vec<usize> {
    return zone
        .split("-")
        .map(|subzone| subzone.parse::<usize>().expect("error parsing subzone"))
        .collect();
}
