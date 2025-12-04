use std::collections::HashSet;
use std::fs;

pub fn solve() -> (u64, u64) {
    let input = fs::read_to_string("input/day04/input.txt").expect("Couldn't open file!");
    let lines: Vec<&str> = input.trim().lines().collect();
    let mut positions: HashSet<(i32, i32)> = parse_grid(&lines);

    let p1 = part_1(&positions);
    let p2 = part_2(&mut positions);

    (p1, p2)
}

fn parse_grid(lines: &[&str]) -> HashSet<(i32, i32)> {
    let mut positions: HashSet<(i32, i32)> = HashSet::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '@' {
                positions.insert((x as i32, y as i32));
            }
        }
    }

    positions
}

fn check_access(positions: &HashSet<(i32, i32)>, location: &(i32, i32)) -> bool {
    let mut adj_num = 0;
    let (x, y) = location;

    // check north
    if positions.contains(&(x.clone(), y - 1)) {
        adj_num += 1;
    }

    // check north-west
    if positions.contains(&(x - 1, y - 1)) {
        adj_num += 1;
    }

    // check north-east
    if positions.contains(&(x + 1, y - 1)) {
        adj_num += 1;
    }

    //check south
    if positions.contains(&(x.clone(), y + 1)) {
        adj_num += 1;
    }

    // check south-west
    if positions.contains(&(x - 1, y + 1)) {
        adj_num += 1;
    }

    // check south-east
    if positions.contains(&(x + 1, y + 1)) {
        adj_num += 1;
    }

    // check east
    if positions.contains(&(x + 1, y.clone())) {
        adj_num += 1;
    }

    // check west
    if positions.contains(&(x - 1, y.clone())) {
        adj_num += 1;
    }

    if adj_num < 4 { true } else { false }
}

fn part_1(positions: &HashSet<(i32, i32)>) -> u64 {
    let mut valid_rolls: u64 = 0;

    for position in positions {
        if check_access(positions, position) {
            valid_rolls += 1;
        }
    }

    valid_rolls
}

fn part_2(positions: &mut HashSet<(i32, i32)>) -> u64 {
    let mut valid_rolls: u64;

    let to_remove: Vec<_> = positions
        .iter()
        .filter(|p| check_access(positions, p))
        .cloned()
        .collect();

    valid_rolls = to_remove.len() as u64;

    for p in &to_remove {
        positions.remove(&p);
    }

    if !to_remove.is_empty() {
        valid_rolls += part_2(positions);
    }

    valid_rolls
}
