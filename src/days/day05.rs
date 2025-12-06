use std::fs;

pub fn solve() -> (u64, u64) {
    let input = fs::read_to_string("input/day05/input.txt").expect("Couldn't open file!");
    let input_parts: Vec<&str> = input.trim().split("\n\n").collect();
    let ranges: Vec<(u64, u64)> = input_parts[0]
        .lines()
        .map(|pair| {
            let mut nums = pair.split('-').map(|x| x.parse::<u64>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .collect();
    let ids: Vec<u64> = input_parts[1]
        .lines()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let p1 = part_1(&ranges, &ids);
    let p2 = part_2(&ranges);

    (p1, p2)
}

fn part_1(ranges: &[(u64, u64)], ids: &Vec<u64>) -> u64 {
    let mut valid_num: u64 = 0;

    for id in ids {
        for (lower, upper) in ranges {
            if (lower..=upper).contains(&id) {
                valid_num += 1;
                break;
            }
        }
    }

    valid_num
}

fn part_2(ranges: &[(u64, u64)]) -> u64 {
    let mut valid_num: u64 = 0;
    let merged_ranges = merge_ranges(ranges);

    for (start, end) in &merged_ranges {
        valid_num += (end + 1) - start;
    }

    valid_num
}

fn merge_ranges(ranges: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let mut sorted_ranges = ranges.to_vec();
    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    sorted_ranges.sort_by_key(|&(start, _)| start);

    merged_ranges.push(sorted_ranges[0]);
    for &(start, end) in &sorted_ranges[1..] {
        let last = merged_ranges.last_mut().unwrap();

        if start <= last.1 {
            last.1 = u64::max(last.1, end)
        } else {
            merged_ranges.push((start, end));
        }
    }

    merged_ranges
}
