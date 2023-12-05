use nom::{
    bytes::complete::{tag, take},
    character::complete::{alpha1, line_ending},
    multi::{count, separated_list1},
    sequence::{preceded, separated_pair},
    IResult,
};
#[derive(Clone)]
struct Valve<'a> {
    name: &'a str,
    flow: i32,
}
fn parse_valve(input: &str) -> IResult<&str, Valve> {
    let (input, name) = preceded(tag("Valve "), take(2 as usize))(input)?;
    let (input, flow) = preceded(tag(" has flow rate="), nom::character::complete::i32)(input)?;

    let valve = Valve { name, flow };

    Ok((input, valve))
}
fn parse_adjacent(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, names) = separated_list1(tag(" ,"), take(2 as usize))(input)?;
    Ok((input, names))
}

fn parse_line(input: &str) -> IResult<&str, (Valve, Vec<&str>)> {
    let (input, (valve, adjacent_valve_names)) = separated_pair(
        parse_valve,
        tag("; tunnels lead to valves "),
        parse_adjacent,
    )(input)?;

    Ok((input, (valve, adjacent_valve_names)))
}
fn parse_input(input: &str) -> IResult<&str, Vec<Valve>> {
    let (_, list_valves_names) = separated_list1(line_ending, parse_line)(input)?;
    let (valves, neighbours): (Vec<Valve>, Vec<Vec<&str>>) = list_valves_names.into_iter().unzip();

    let neighbours = neighbours
        .iter()
        .map(|sublist| {
            sublist
                .iter()
                .filter_map(|name| {
                    for valve in valves.iter() {
                        if valve.name == *name {
                            return Some(valve);
                        }
                    }
                    return None;
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    todo!()
}

pub fn process_part_1(input: &str) -> usize {
    let (_, valve_list) = parse_input(input).unwrap();
    todo!()
}
pub fn process_part_2(_input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
    #[test]
    fn it_works_part1() {
        let result = process_part_1(INPUT);
        assert_eq!(result, 31);
    }
    #[test]
    #[ignore]
    fn it_works_part2() {
        let result = process_part_2(INPUT);
        assert_eq!(result, 29);
    }
}
