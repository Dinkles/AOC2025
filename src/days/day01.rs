use std::fs;

pub fn solve() -> (u64, u64) {
    let input = fs::read_to_string("input/day01/input.txt").expect("Couldn't open file!");
    let lines: Vec<&str> = input.trim().lines().collect();

    let p1 = part_1(&lines);
    let p2 = part_2(&lines);

    (p1, p2)
}

fn part_1(lines: &Vec<&str>) -> u64 {
    let mut dial: i16 = 50;
    let mut zero_num: u64 = 0;

    for line in lines {
        let amount = parse_instruction(line);

        dial = (dial + 100 + amount).rem_euclid(100);

        if dial == 0 {
            zero_num += 1;
        }
    }

    zero_num
}

// brute force because I couldn't figure out a better solution
fn part_2(lines: &Vec<&str>) -> u64 {
    let mut dial: i16 = 50;
    let mut zero_num: u64 = 0;

    for line in lines {
        let amount = parse_instruction(line);
        let direction = if amount < 0 { -1 } else { 1 };

        // move dial by 1 each time
        for _ in 0..amount.abs() {
            dial = (dial + direction * 1).rem_euclid(100);
            if dial == 0 {
                zero_num += 1;
            }
        }
    }

    return zero_num;
}

fn parse_instruction(instruction: &str) -> i16 {
    let (str_dir, str_amount) = instruction.split_at(1);
    let char_dir: char = str_dir.parse().expect("Couldn't parse!");
    let num_amount: i16 = str_amount.parse().expect("Couldn't parse!");

    if char_dir == 'L' {
        num_amount * -1
    } else {
        num_amount
    }
}
