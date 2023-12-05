use std::fs;
fn main() {
    let content = fs::read_to_string("input.txt").expect("Cannot load the file");
    let lines = content.lines();
    let overlaps: usize = lines
        .map(|line| {
            let mut splitted = line.split(",");
            let zone_one = splitted.next().unwrap();
            let zone_two = splitted.next().unwrap();
            let are_contained = is_contained(zone_one, zone_two);
            println!("{}", are_contained);
            if are_contained {
                return 1;
            } else {
                return 0;
            }
        })
        .sum();
    println!("{}", overlaps);
}

fn is_contained(zone_one: &str, zone_two: &str) -> bool {
    let boundaries_one = zone_boundaries(zone_one);
    let boundaries_two = zone_boundaries(zone_two);
    println!("{:?}, {:?}", boundaries_one, boundaries_two);

    if boundaries_one[0] <= boundaries_two[0] && boundaries_one[1] >= boundaries_two[1] {
        return true;
    } else if boundaries_one[0] >= boundaries_two[0] && boundaries_one[1] <= boundaries_two[1] {
        return true;
    } else {
        return false;
    }
}
fn zone_boundaries(zone: &str) -> [usize; 2] {
    let mut x = zone.split("-");
    let x1 = x
        .next()
        .unwrap()
        .parse::<usize>()
        .expect("Error parsing zone to int");
    let x2 = x
        .next()
        .unwrap()
        .parse::<usize>()
        .expect("Error parsing zone to int");
    return [x1, x2];
}
