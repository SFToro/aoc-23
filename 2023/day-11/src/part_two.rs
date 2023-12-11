use std::collections::BTreeMap;

pub fn parse(input: &str) -> (BTreeMap<(usize, usize), char>, Vec<usize>, Vec<usize>) {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let rows = map
        .iter()
        .enumerate()
        .filter_map(|(i, row)| row.iter().all(|v| *v == '.').then_some(i))
        .collect();

    let columns = transpose(&map)
        .iter()
        .enumerate()
        .filter_map(|(i, column)| column.iter().all(|v| *v == '.').then_some(i))
        .collect();

    let map: BTreeMap<_, _> = map
        .into_iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.into_iter()
                .enumerate()
                .map(|(j, v)| ((i, j), v))
                .collect::<Vec<_>>()
        })
        .collect();
    (map, rows, columns)
}

fn transpose(data: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let size_x = data.len();
    let size_y = data[0].len();
    let mut transposed = Vec::new();
    for j in 0..size_y {
        let mut temp = Vec::new();
        for i in 0..size_x {
            temp.push(data[i][j])
        }
        transposed.push(temp);
    }
    transposed
}
