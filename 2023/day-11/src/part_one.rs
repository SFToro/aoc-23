pub fn print(data: Vec<Vec<char>>) -> String {
    let mut result = String::new();
    data.iter().for_each(|row| {
        row.iter().for_each(|v| result += &(format!("{}", v)));
        result += "\n"
    });
    result
}

pub fn expand(input: &str) -> Vec<Vec<char>> {
    let mut map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut size_x = map.len();
    let mut size_y = map[0].len();

    let mut i = 0;
    while i < size_x {
        if map[i].iter().all(|v| *v == '.') {
            println!("line {} is all dots", i);
            let items = std::iter::repeat('.').take(size_y).collect::<Vec<_>>();
            map.insert(i, items);
            size_x += 1;
            i += 1;
        }
        i += 1;
    }

    let mut transposed_map = transpose(map.clone());

    size_x = transposed_map.len();
    size_y = transposed_map[0].len();
    let mut i = 0;
    while i < size_y {
        if transposed_map[i].iter().all(|v| *v == '.') {
            println!("column {} is all dots", i);
            let items = std::iter::repeat('.').take(size_y).collect::<Vec<_>>();
            transposed_map.insert(i, items);
            size_x += 1;
            i += 1;
        }
        i += 1;
    }
    map
}

fn transpose(data: Vec<Vec<char>>) -> Vec<Vec<char>> {
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
