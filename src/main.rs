mod days;

use days::{day01, day02, day03, day04, day05};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Enter the day(s) run.");
        println!("Example: adventofcode2025 1 2 3");
        return;
    }

    let days: Vec<u8> = args[1..]
        .iter()
        .map(|x| {
            x.parse()
                .unwrap_or_else(|v| panic!("Not a valid day: {}\n Valid days: 1-12", v))
        })
        .collect();

    for day in days {
        let solve_function = get_solver_function(day);

        println!("========== Day: {} ==========", day);
        let (p1, p2) = solve_function();
        println!("Part 1: {}", p1);
        println!("Part 2: {}", p2);
        print!("\n\n")
    }
}

fn get_solver_function(day: u8) -> fn() -> (u64, u64) {
    match day {
        1 => day01::solve,
        2 => day02::solve,
        3 => day03::solve,
        4 => day04::solve,
        5 => day05::solve,
        _ => unimplemented!(),
    }
}
