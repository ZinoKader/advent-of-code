use std::fs;

fn main() {
    println!("{}", part_1());
    println!("{}", part_2());
}

fn part_1() -> i32 {
    let input: Vec<(String, i32)> = fs::read_to_string("data/input")
        .unwrap()
        .to_string()
        .split("\n")
        .map(|x| {
            let l: Vec<&str> = x.split_whitespace().collect();
            (l[0].to_owned(), l[1].parse::<i32>().unwrap())
        })
        .collect();

    let mut horizontal: i32 = 0;
    let mut vertical: i32 = 0;
    for (direction, delta) in input {
        match direction.as_str() {
            "up" => vertical -= delta,
            "down" => vertical += delta,
            "forward" => horizontal += delta,
            _ => (),
        }
    }

    horizontal * vertical
}

fn part_2() -> i32 {
    let input: Vec<(String, i32)> = fs::read_to_string("data/input")
        .unwrap()
        .to_string()
        .split("\n")
        .map(|x| {
            let l: Vec<&str> = x.split_whitespace().collect();
            (l[0].to_owned(), l[1].parse::<i32>().unwrap())
        })
        .collect();

    let mut horizontal: i32 = 0;
    let mut vertical: i32 = 0;
    let mut aim: i32 = 0;
    for (direction, delta) in input {
        match direction.as_str() {
            "up" => aim -= delta,
            "down" => aim += delta,
            "forward" => {
                horizontal += delta;
                vertical += aim * delta;
            }
            _ => (),
        }
    }

    horizontal * vertical
}
