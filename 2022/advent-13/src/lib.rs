use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};
#[derive(Debug)]
struct Pair {
    left: Packet,
    right: Packet,
}

impl Pair {
    fn new(left: Packet, right: Packet) -> Self {
        Self { left, right }
    }
}
#[derive(Debug)]
enum Packet {
    List(Vec<Packet>),
    Number(i32),
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::List(l0), Self::List(l1)) => l0.partial_cmp(l1),
            (Self::Number(l0), Self::Number(l1)) => l0.partial_cmp(l1),
            (Self::List(l0), Self::Number(l1)) => l0.partial_cmp(&vec![Packet::Number(*l1)]),
            (Self::Number(l0), Self::List(l1)) => l1.partial_cmp(&vec![Packet::Number(*l0)]),
        }
    }
}
impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(l1)) => l0 == l1,
            (Self::Number(l0), Self::Number(l1)) => l0 == l1,
            (Self::List(l0), Self::Number(l1)) => l0 == &vec![Packet::Number(*l1)],
            (Self::Number(l0), Self::List(l1)) => &vec![Packet::Number(*l0)] == l1,
        }
    }
}
// [1,1,3,1,1]
// [1,1,5,1,1]
// This is a pair. Each member of the pair is a packet. A packet has two variants. A packet is either a list of packets or a number.
// That implies that the packet can contain itself recursively. However, the outermost packet is always a list (delimited by "[","]").
//The first packet of the pair is a list and the second one is a list too. Each list contains 5 packets of the number variant.

// [9]
// [[8,7,6]]
// As stated before each member of the pair is a packet of type list.
// The first member is a list of a single Packet::number variant.
// The second member is a list of a single Packet::list variant. At the same time, this list contains 3 packets of the number variant.
fn parse_packets(input: &str) -> IResult<&str, Packet> {
    let (input, output) = delimited(
        tag("["),
        separated_list0(
            // separated_list1(
            tag(","),
            alt((
                nom::character::complete::i32.map(Packet::Number),
                parse_packets,
                // tag("").map(|_| Packet::List(vec![])),
            )),
        ),
        tag("]"),
    )(input)?;

    Ok((input, Packet::List(output)))
}

fn parse_pair(input: &str) -> IResult<&str, Pair> {
    let (input, packet_tuple) = separated_pair(parse_packets, newline, parse_packets)(input)?;
    // dbg!(&packet_tuple);
    Ok((input, Pair::new(packet_tuple.0, packet_tuple.1)))
}
fn parse_pairs(input: &str) -> IResult<&str, Vec<Pair>> {
    let (input, vec_of_pairs) = separated_list1(tag("\n\n"), parse_pair)(input)?;
    Ok((input, vec_of_pairs))
}
pub fn process_part_1(input: &str) -> usize {
    let (_input, vec_of_pairs) = parse_pairs(input).unwrap();
    todo!()
}
pub fn process_part_2(_input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
    #[test]
    fn it_compares_recursively() {
        let (_, pairs) = parse_pairs(INPUT).unwrap();
        assert_eq!(pairs[6].left > pairs[6].right, true)
    }
    #[test]
    fn it_compares() {
        let (_, pairs) = parse_pairs(INPUT).unwrap();
        assert_eq!(pairs[0].left < pairs[0].right, true)
    }
    #[test]
    #[ignore]
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
