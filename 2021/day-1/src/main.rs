use std::fs;

fn main() {
    println!("{}", part_1());
    println!("{}", part_2());
}

fn part_1() -> i32 {
    let a: Vec<i32> = fs::read_to_string("data/input")
        .unwrap()
        .to_string()
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut increases = 0;
    for i in 1..a.len() {
        if a[i] > a[i - 1] {
            increases += 1;
        }
    }
    increases
}

fn part_2() -> i32 {
    let a: Vec<i32> = fs::read_to_string("data/input")
        .unwrap()
        .to_string()
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut increases = 0;
    for i in 3..a.len() {
        if a[i] > a[i - 3] {
            increases += 1;
        }
    }
    increases
}
