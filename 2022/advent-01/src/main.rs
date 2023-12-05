use std::fs;
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Cannot load the file");
    let counts = contents.split("\n\n").map(|chunk| -> usize {
        chunk
            .split("\n")
            .map(|line| line.parse().unwrap_or(0))
            .sum()
    });

    let mut v = counts.collect::<Vec<_>>();
    v.sort();
    let idx = v.len() - 1;
    println!("{}, {}", idx, v[idx]);

    let mut sum = 0;
    for i in 0..=2 {
        println!("{}, {}", i, v[idx - i]);
        sum += v[idx - i];
    }
    println!("{}", sum);
    println!("{:?}", &v[idx - 2..=idx].iter().sum::<usize>());
}
