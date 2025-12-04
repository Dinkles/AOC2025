use std::{char::from_digit, fs};

pub fn solve() -> (u64, u64) {
    let input = fs::read_to_string("input/day03/input.txt").expect("Couldn't open file!");
    let lines: Vec<&str> = input.trim().lines().collect();

    let p1 = part_1(&lines);
    let p2 = part_2(&lines);

    (p1, p2)
}

fn part_1(lines: &[&str]) -> u64 {
    let mut joltage_sum: u64 = 0;

    for line in lines {
        let mut num1: u32 = 0;
        let mut num2: u32 = 0;
        let mut start_pos: usize = 0;

        for (i, c) in line[..line.len() - 1].chars().enumerate() {
            let digit = c.to_digit(10).unwrap();
            if digit > num1 {
                num1 = digit;
                start_pos = i + 1;
            }
        }

        for c in line[start_pos..].chars() {
            let digit = c.to_digit(10).unwrap();
            if digit > num2 {
                num2 = digit;
            }
        }

        joltage_sum += u64::from((num1 * 10) + num2);
    }

    joltage_sum
}

fn part_2(lines: &[&str]) -> u64 {
    let mut joltage_sum: u64 = 0;

    for line in lines {
        let mut joltage: String = String::new();
        let mut start_pos: usize = 0;

        for n in (0..12).rev() {
            let mut num = 0;
            let mut offset = 0;
            let end_pos = line.len() - n;

            for (i, c) in line[start_pos..end_pos].chars().enumerate() {
                let digit = c.to_digit(10).unwrap();
                if digit > num {
                    num = digit;
                    offset = i;
                }
            }

            start_pos += offset + 1;
            joltage.push(from_digit(num, 10).unwrap());
        }

        joltage_sum += joltage.parse::<u64>().unwrap();
    }

    joltage_sum
}
