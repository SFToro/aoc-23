use color_eyre::eyre::Context;
use std::{collections::HashSet, fs, str::FromStr};

#[derive(Debug)]
enum Movement {
    Up { amount: isize },
    Down { amount: isize },
    Right { amount: isize },
    Left { amount: isize },
}
impl FromStr for Movement {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let  Some((direction,amount)) = s.split_once(" ") else {
            return Err(color_eyre::eyre::eyre!("Expected <direction>SP<amount>EOF, got {s:?}"));
    };
        let parsed_amount = amount.parse::<isize>().unwrap_or(0);
        match direction {
            "U" => Ok(Movement::Up {
                amount: parsed_amount,
            }),
            "D" => Ok(Movement::Down {
                amount: parsed_amount,
            }),
            "R" => Ok(Movement::Right {
                amount: parsed_amount,
            }),
            "L" => Ok(Movement::Left {
                amount: parsed_amount,
            }),
            _ => Err(color_eyre::eyre::eyre!(
                "Not a valid direction: {direction:?}"
            )),
        }
    }
}

#[derive(Default, Debug)]
struct Pos {
    x: isize,
    y: isize,
    vec_registry: Vec<(isize, isize)>,
    registry: HashSet<(isize, isize)>,
}
impl Pos {
    fn move_movement(&mut self, movement: Movement) {
        match movement {
            Movement::Up { amount } => {
                for _ in 1..=amount {
                    self.y += 1
                }
            }
            Movement::Down { amount } => {
                for _ in 1..=amount {
                    self.y -= 1
                }
            }

            Movement::Left { amount } => {
                for _ in 1..=amount {
                    self.x -= 1
                }
            }

            Movement::Right { amount } => {
                for _ in 1..=amount {
                    self.x += 1
                }
            }
        }
    }
    fn follow(&mut self, forward: &Pos) {
        let (x, y) = Self::sub(forward, self);
        println!("{x},{y}");
        if x == 0 && y == 0 {
            self.register_pos();
            return;
        } else if x.abs() == 1 && y.abs() == 1 {
            self.register_pos();
            return;
        } else if y == 0 {
            for _ in 1..x.abs() {
                self.x += 1 * x.signum();
                self.register_pos();
            }
        } else if x == 0 {
            for _ in 1..y.abs() {
                self.y += 1 * y.signum();
                self.register_pos();
            }
        } else if x.abs() == 1 {
            self.x += 1 * x.signum();
            for _ in 1..y.abs() {
                self.y += 1 * y.signum();
                self.register_pos();
            }
        } else if y.abs() == 1 {
            self.y += 1 * y.signum();
            for _ in 1..x.abs() {
                self.x += 1 * x.signum();
                self.register_pos();
            }
        } else if y.abs() == 2 && x.abs() == 2 {
            self.y += 1 * y.signum();
            self.x += 1 * x.signum();
            self.register_pos();
        }
    }

    fn sub(minu: &Pos, rhs: &Pos) -> (isize, isize) {
        (minu.x - rhs.x, minu.y - rhs.y)
    }
    fn register_pos(&mut self) {
        self.registry.insert((self.x, self.y));
        self.vec_registry.push((self.x, self.y));
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut head_pos = Pos::default();

    let mut knot_1 = Pos::default();
    let mut knot_2 = Pos::default();
    let mut knot_3 = Pos::default();
    let mut knot_4 = Pos::default();
    let mut knot_5 = Pos::default();
    let mut knot_6 = Pos::default();
    let mut knot_7 = Pos::default();
    let mut knot_8 = Pos::default();
    let mut tail_pos = Pos::default();
    tail_pos.register_pos();

    let contents = read_input()?;
    contents.lines().for_each(|l| {
        let movement = l.parse::<Movement>().unwrap();

        match movement {
            Movement::Up { amount } => {
                for _ in 1..=amount {
                    head_pos.y += 1;
                    knot_1.follow(&head_pos);
                    knot_2.follow(&knot_1);
                    knot_3.follow(&knot_2);
                    knot_4.follow(&knot_3);
                    knot_5.follow(&knot_4);
                    knot_6.follow(&knot_5);
                    knot_7.follow(&knot_6);
                    knot_8.follow(&knot_7);
                    tail_pos.follow(&knot_8);
                }
            }
            Movement::Down { amount } => {
                for _ in 1..=amount {
                    head_pos.y -= 1;
                    knot_1.follow(&head_pos);
                    knot_2.follow(&knot_1);
                    knot_3.follow(&knot_2);
                    knot_4.follow(&knot_3);
                    knot_5.follow(&knot_4);
                    knot_6.follow(&knot_5);
                    knot_7.follow(&knot_6);
                    knot_8.follow(&knot_7);
                    tail_pos.follow(&knot_8)
                }
            }

            Movement::Left { amount } => {
                for _ in 1..=amount {
                    head_pos.x -= 1;
                    knot_1.follow(&head_pos);
                    knot_2.follow(&knot_1);
                    knot_3.follow(&knot_2);
                    knot_4.follow(&knot_3);
                    knot_5.follow(&knot_4);
                    knot_6.follow(&knot_5);
                    knot_7.follow(&knot_6);
                    knot_8.follow(&knot_7);
                    tail_pos.follow(&knot_8)
                }
            }

            Movement::Right { amount } => {
                for _ in 1..=amount {
                    head_pos.x += 1;
                    knot_1.follow(&head_pos);
                    knot_2.follow(&knot_1);
                    knot_3.follow(&knot_2);
                    knot_4.follow(&knot_3);
                    knot_5.follow(&knot_4);
                    knot_6.follow(&knot_5);
                    knot_7.follow(&knot_6);
                    knot_8.follow(&knot_7);
                    tail_pos.follow(&knot_8)
                }
            }
        }

        // println!("head 1: {:?}, {}", head_pos.x, head_pos.y);
        // println!("knot 1: {:?}, {}", knot_1.x, knot_1.y);
        // println!("knot 2: {:?}, {}", knot_2.x, knot_2.y);

        // println!("tail: {:?}, {}", tail_pos.x, tail_pos.y);
    });

    dbg!(tail_pos.registry.len());
    // println!("{:?}", tail_pos.vec_registry);
    Ok(())
}
fn read_input() -> color_eyre::Result<String> {
    let contents = fs::read_to_string("input.txt").wrap_err("reading src/input.txt")?;
    Ok(contents)
}
