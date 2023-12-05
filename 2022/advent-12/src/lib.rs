use itertools::Itertools;
use petgraph::{algo::astar, prelude::DiGraphMap};

pub fn process_part_1(input: Vec<u8>) -> usize {
    let dim_x = input.iter().position(|x| *x == b'\n').unwrap();
    let input: Vec<_> = input.iter().filter(|x| **x != b'\n').collect();
    let idz = input.iter().position(|x| **x == b'E').unwrap();
    let end = (idz / dim_x, (idz % dim_x));
    let id0 = input.iter().position(|x| **x == b'S').unwrap();
    let start = (id0 / dim_x, (id0 % dim_x));
    dbg!(start);
    dbg!(end);

    let matrix: Vec<&[&u8]> = input.chunks(dim_x).collect();

    let coords: Vec<_> = (0..matrix.len() as i32)
        .cartesian_product(0..dim_x as i32)
        .collect();
    let mut edges: Vec<Vec<((i32, i32), (i32, i32))>> = vec![];
    for (x, y) in coords {
        let neighbors = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
        let current_node = matrix[x as usize][y as usize];
        let connections: Vec<_> = neighbors
            .iter()
            .filter_map(|cell| {
                matrix.get(cell.0 as usize).map(|row| {
                    row.get(cell.1 as usize).and_then(|neighbor_node| {
                        if *current_node == b'S'
                            || current_node + 1 >= **neighbor_node && **neighbor_node != b'E'
                            || (**neighbor_node == b'E' && *current_node == b'z')
                            || (*current_node == b'E')
                        {
                            Some(((x, y), cell.to_owned()))
                        } else {
                            None
                        }
                    })
                })
            })
            .flatten()
            .collect();
        edges.push(connections);
    }
    let edges = edges.into_iter().flatten().collect::<Vec<_>>();

    let graph = DiGraphMap::<(i32, i32), ()>::from_edges(edges);
    let f = (end.0 as i32, end.1 as i32);
    let a = (start.0 as i32, start.1 as i32);
    let path = astar(&graph, a, |finish| finish == f, |_| 1, |_| 0);
    if let Some(result) = path {
        result.0
    } else {
        0
    }
}
pub fn process_part_2(input: Vec<u8>) -> usize {
    let dim_x = input.iter().position(|x| *x == b'\n').unwrap();
    let input: Vec<_> = input.iter().filter(|x| **x != b'\n').collect();
    let idz = input.iter().position(|x| **x == b'E').unwrap();
    let end = (idz / dim_x, (idz % dim_x));
    let id0 = input.iter().position(|x| **x == b'S').unwrap();
    let start = (id0 / dim_x, (id0 % dim_x));
    dbg!(start);
    dbg!(end);

    let matrix: Vec<&[&u8]> = input.chunks(dim_x).collect();

    let coords: Vec<_> = (0..matrix.len() as i32)
        .cartesian_product(0..dim_x as i32)
        .collect();
    let mut edges: Vec<Vec<((i32, i32), (i32, i32))>> = vec![];
    for (x, y) in coords {
        let neighbors = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
        let current_node = matrix[x as usize][y as usize];
        let connections: Vec<_> = neighbors
            .iter()
            .filter_map(|cell| {
                matrix.get(cell.0 as usize).map(|row| {
                    row.get(cell.1 as usize).and_then(|neighbor_node| {
                        if *current_node == b'S'
                            || current_node + 1 >= **neighbor_node && **neighbor_node != b'E'
                            || (**neighbor_node == b'E' && *current_node == b'z')
                            || (*current_node == b'E')
                        {
                            Some(((x, y), cell.to_owned()))
                        } else {
                            None
                        }
                    })
                })
            })
            .flatten()
            .collect();
        edges.push(connections);
    }
    let edges = edges.into_iter().flatten().collect::<Vec<_>>();

    let graph = DiGraphMap::<(i32, i32), ()>::from_edges(edges);
    let letters_a = matrix
        .iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(y, v)| if **v == b'a' { Some((x, y)) } else { None })
        })
        .collect::<Vec<_>>();
    let f = (end.0 as i32, end.1 as i32);

    letters_a
        .iter()
        .filter_map(|(x, y)| {
            let a = (*x as i32, *y as i32);
            let path = astar(&graph, a, |finish| finish == f, |_| 1, |_| 0);
            if let Some(result) = path {
                Some(result.0)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &[u8; 44] = include_bytes!("../input-test.txt");
    #[test]
    fn it_works_part1() {
        let result = process_part_1(INPUT.to_vec());
        assert_eq!(result, 31);
    }
    #[test]
    fn it_works_part2() {
        let result = process_part_2(INPUT.to_vec());
        assert_eq!(result, 29);
    }
}
