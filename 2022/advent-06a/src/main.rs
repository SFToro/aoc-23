use std::collections::HashSet;

fn main() {
    let contents = include_bytes!("../input-test.txt");
    let mut i = 0;
    let mut found = false;
    let mut sliding_window = contents.windows(14);
    for chunk in sliding_window {
        i += 1;
        let mut my_set = HashSet::new();
        for i in chunk {
            my_set.insert(i);
        }
        if my_set.len() == 14 {
            println!("{}", i + 14);
            println!("{}", my_set.len());
            break;
        }
    }
}
