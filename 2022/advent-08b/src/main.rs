use std::fs;
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error cargando el archivo");
    let lines = contents.lines();
    let matrix = lines
        .map(|l| {
            l.chars()
                .map(|v| v.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut max_score = 0;

    for (i, row) in matrix.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let row_wise = check_row(&matrix, i, j, row.len());
            let column_wise = check_column(&matrix, i, j);
            if row_wise * column_wise > max_score {
                max_score = column_wise * row_wise;
            }
            if row_wise * column_wise != 0 {
                println!(
                    "Matrix[{}][{}], {}: {}*{}={}",
                    i,
                    j,
                    matrix[i][j],
                    row_wise,
                    column_wise,
                    row_wise * column_wise
                )
            }
        }
    }
    println!("{}", max_score);
    // println!("{}", check_row(&matrix, 3, 2, 5));
    // println!("{}", check_column(&matrix, 3, 2));
}

fn check_column(matrix: &Vec<Vec<u32>>, i: usize, j: usize) -> usize {
    let mut top = 0;
    let mut bottom = 0;
    for k in (0..i).rev() {
        // println!("Matrix[{}][{}] = {}", k, j, matrix[k][j]);
        if matrix[k][j] < matrix[i][j] {
            top += 1;
        } else if matrix[k][j] >= matrix[i][j] {
            top += 1;
            break;
        } else {
            break;
        }
    }
    for k in i + 1..matrix.len() {
        // println!("Matrix[{}][{}] = {}", k, j, matrix[k][j]);
        if matrix[k][j] < matrix[i][j] {
            bottom += 1;
        } else if matrix[k][j] >= matrix[i][j] {
            bottom += 1;
            break;
        } else {
            break;
        }
    }
    top * bottom
}
fn check_row(matrix: &Vec<Vec<u32>>, i: usize, j: usize, row_len: usize) -> usize {
    let mut right = 0;
    let mut left = 0;
    for k in (0..j).rev() {
        // println!("Matrix[{}][{}] = {}", i, k, matrix[i][k]);
        if matrix[i][k] < matrix[i][j] {
            left += 1;
        } else if matrix[i][k] >= matrix[i][j] {
            left += 1;
            break;
        } else {
            break;
        }
    }
    for k in j + 1..row_len {
        // println!("Matrix[{}][{}] = {}", i, k, matrix[i][k]);
        if matrix[i][k] < matrix[i][j] {
            right += 1;
        } else if matrix[i][k] >= matrix[i][j] {
            right += 1;
            break;
        } else {
            break;
        }
    }
    // dbg!(left, right);
    left * right
}
