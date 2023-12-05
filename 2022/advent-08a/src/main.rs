use std::fs;
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error cargando el archivo");
    let lines = contents.lines();
    let my_def: i32 = Default::default();
    println!("{}", my_def);
    const RADIX: u32 = 10;
    let matrix = lines
        .map(|l| {
            l.chars()
                .map(|v| v.to_digit(RADIX).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut count = matrix.len() * 4 - 4;
    println!("{}", matrix.len());
    for (i, row) in matrix.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if i == 0 || i == matrix.len() - 1 || j == 0 || j == row.len() - 1 {
                continue;
            }
            let mut right = true;
            let mut left = true;
            let mut top = true;
            let mut bottom = true;
            for k in 0..j {
                if matrix[i][k] >= matrix[i][j] {
                    left = false;
                    break;
                }
            }
            for k in j + 1..row.len() {
                if matrix[i][k] >= matrix[i][j] {
                    right = false;
                    break;
                }
            }
            for k in 0..i {
                if matrix[k][j] >= matrix[i][j] {
                    top = false;
                    break;
                }
            }
            for k in i + 1..matrix.len() {
                if matrix[k][j] >= matrix[i][j] {
                    bottom = false;
                    break;
                }
            }
            let step_visible = right || left || top || bottom;
            if step_visible {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
