use std::fs;

pub fn solve() -> (u64, u64) {
    let input = fs::read_to_string("input/day02/input.txt").expect("Couldn't open file!");
    let ranges: Vec<(u64, u64)> = input
        .trim()
        .split(',')
        .map(|pair| {
            let mut nums = pair.split('-').map(|x| x.parse::<u64>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .collect();

    let p1 = part_1(&ranges);
    let p2 = part_2(&ranges);

    (p1, p2)
}

fn part_1(ranges: &[(u64, u64)]) -> u64 {
    let mut invalid_sum: u64 = 0;

    for &(start, end) in ranges {
        for num in start..=end {
            let s = num.to_string();
            if s.len() % 2 == 0 {
                let (left, right) = s.split_at(s.len() / 2);
                if left == right {
                    invalid_sum += num;
                }
            }
        }
    }

    invalid_sum
}

fn part_2(ranges: &[(u64, u64)]) -> u64 {
    let mut invalid_sum: u64 = 0;

    for &(start, end) in ranges {
        for num in start..=end {
            let s = num.to_string();
            let doubled = s.repeat(2);
            let middle = &doubled[1..(2 * s.len() - 1)];
            if middle.contains(&s) {
                invalid_sum += num;
            }
        }
    }

    invalid_sum
}
