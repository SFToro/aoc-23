use std::{fs, vec};
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error loading puzzle inputs");
    let n_boxes = contents.split("\n").next().unwrap().as_bytes().len();
    let positions = boxes_position(n_boxes);
    let height = get_height(&contents);
    let mut stacks = build_stacks(height, positions, &contents);
    let orders = get_orders(&contents);

    for order in orders {
        let pops = order[0];
        let from = order[1];
        let to = order[2];
        let mut buffer: Vec<char> = vec![];
        for _ in 0..pops {
            let popped = stacks[from - 1].pop().unwrap();
            buffer.push(popped);
        }

        buffer.reverse();
        for i in 0..pops {
            stacks[to - 1].push(buffer[i]);
        }
    }
    println!("{:?}", stacks);
    for stack in &stacks {
        print!("{}", stack.get(stack.len() - 1).unwrap());
    }
}

fn boxes_position(length: usize) -> Vec<usize> {
    let mut positions: Vec<usize> = vec![1];
    let upper_limit = (length + 2) / 4;
    for i in 1..upper_limit {
        positions.push(i * 4 + 1);
    }
    positions
}

fn get_height(contents: &str) -> usize {
    let mut i = 0;
    for line in contents.lines() {
        if line.as_bytes()[1] == 49 {
            break;
        }
        i += 1;
    }
    i
}

fn build_stacks(height: usize, positions: Vec<usize>, contents: &str) -> Vec<Vec<char>> {
    let mut lines = contents.lines();
    let mut stacks: Vec<Vec<char>> = vec![];
    for _ in &positions {
        stacks.push(vec![]);
    }
    for _ in 0..height {
        let line = lines.next().unwrap().as_bytes();
        let mut i = 0;
        for position in &positions {
            if line[*position] != 32 {
                stacks[i].push(line[*position] as char)
            }
            i += 1;
        }
    }
    let mut i = 0;

    for _ in &positions {
        stacks[i].reverse();
        i += 1;
    }
    stacks
}

fn get_orders(contents: &str) -> Vec<Vec<usize>> {
    let lines = contents.lines();
    let mut orders: Vec<_> = vec![];

    lines.for_each(|line| {
        if &109 == line.as_bytes().get(0).unwrap_or(&0) {
            let mut it = line.split_whitespace().skip(1).step_by(2);
            let mut order: Vec<usize> = vec![];
            order.push(it.next().unwrap().parse::<usize>().expect("Problem"));
            order.push(it.next().unwrap().parse::<usize>().expect("Problem"));
            order.push(it.next().unwrap().parse::<usize>().expect("Problem"));
            orders.push(order);
        }
    });
    orders
}
