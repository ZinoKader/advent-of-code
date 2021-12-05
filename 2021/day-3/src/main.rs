use std::fs;

fn main() {
    println!("{}", part_1());
    println!("{}", part_2());
}

fn part_1() -> i32 {
    let input: Vec<String> = fs::read_to_string("data/input")
        .unwrap()
        .to_string()
        .split("\n")
        .map(|x| x.to_string())
        .collect();

    let mut most_common = vec![0; input[0].len()];
    for line in input {
        let chars: Vec<char> = line.chars().collect();
        for i in 0..line.len() {
            let mut offset = chars[i].to_digit(10).unwrap() as i32;
            if offset == 0 {
                offset = -1;
            }
            most_common[i] += offset;
        }
    }

    let gamma: i32 = most_common
        .iter()
        .map(|num| if *num > 0 { 1 } else { 0 })
        .rev()
        .enumerate()
        .map(|(i, num)| num * 2_i32.pow(i as u32))
        .sum();
    let epsilon = gamma ^ 0xFFF;

    gamma * epsilon
}

fn part_2() -> i32 {
    let input: Vec<String> = fs::read_to_string("data/input")
        .unwrap()
        .to_string()
        .split("\n")
        .map(|x| x.to_string())
        .collect();

    fn calc_most_common(input: &Vec<String>, pos: usize) -> i32 {
        let mut most_common = 0;
        for line in input {
            let chars: Vec<char> = line.chars().collect();
            let mut offset = chars[pos].to_digit(10).unwrap() as i32;
            if offset == 0 {
                offset = -1;
            }
            most_common += offset;
        }
        return if most_common >= 0 { 1 } else { 0 };
    }

    fn keep_common_pos(kept: Vec<String>, pos: usize, flipped: bool) -> i32 {
        let mut mc_bit = calc_most_common(&kept, pos);
        if flipped {
            mc_bit ^= 0b1;
        }
        let to_keep: Vec<String> = kept
            .into_iter()
            .filter(|line| {
                let line_pos_bit = line.chars().collect::<Vec<char>>()[pos]
                    .to_digit(10)
                    .unwrap() as i32;
                line_pos_bit == mc_bit
            })
            .collect();

        return if to_keep.len() == 1 {
            // base case
            to_keep[0]
                .chars()
                .rev()
                .enumerate()
                .map(|(i, num)| num.to_digit(10).unwrap() as i32 * 2_i32.pow(i as u32))
                .sum()
        } else {
            keep_common_pos(to_keep, pos + 1, flipped) // recursion case
        };
    }

    let oxygen_rating = keep_common_pos(input.to_vec(), 0, false);
    let scrubber_rating = keep_common_pos(input.to_vec(), 0, true);

    oxygen_rating * scrubber_rating
}
