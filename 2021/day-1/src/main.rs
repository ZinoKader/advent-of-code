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
    for i in 2..a.len() - 1 {
        let w1_sum = a[i - 2] + a[i - 1] + a[i];
        let w2_sum = a[i - 1] + a[i] + a[i + 1];
        if w2_sum > w1_sum {
            increases += 1;
        }
    }
    increases
}
