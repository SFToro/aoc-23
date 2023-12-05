use std::{collections::HashMap, fs};
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error loading file");
    let lines = contents.lines();

    let mut sum = 0;
    let mut my_path_vector: Vec<String> = vec![];
    let mut my_hash_map: HashMap<String, usize> = HashMap::new();
    for line in lines {
        let mut it = line.split_whitespace();
        let first_word = it.next().unwrap();
        let second_word = it.next().unwrap();
        if first_word == "$" && second_word == "cd" {
            let third_word = it.next().unwrap();
            if third_word == ".." {
                for path in &my_path_vector {
                    if let Some(amount) = my_hash_map.get_mut(path) {
                        *amount += sum;
                    }
                }
                my_path_vector.pop();
                sum = 0;
            } else {
                for path in &my_path_vector {
                    if let Some(amount) = my_hash_map.get_mut(path) {
                        *amount += sum;
                    }
                }
                if my_hash_map.contains_key(third_word) {
                    let mut index_try: String = String::from(third_word);
                    while my_hash_map.contains_key(&index_try) {
                        index_try.push('0');
                    }
                    println!("{}, {}", third_word, index_try);
                    my_path_vector.push(index_try.clone());
                    my_hash_map.insert(index_try, 0);
                } else {
                    my_path_vector.push(third_word.to_string());
                    my_hash_map.insert(third_word.to_string(), 0);
                }
                sum = 0;
            };
        } else if first_word.parse::<usize>().unwrap_or(0) != 0 {
            sum += first_word.parse::<usize>().unwrap_or(0);
        }
    }

    for path in my_path_vector {
        if let Some(amount) = my_hash_map.get_mut(&path) {
            *amount += sum;
        }
    }
    // println!("{:?}", my_hash_map.get_key_value("/"));
    // println!("{:?}", my_hash_map);
    println!(
        "{}",
        my_hash_map
            .into_values()
            .filter(|v| v < &100000)
            .sum::<usize>()
    );
}
