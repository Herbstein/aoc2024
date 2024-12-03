use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|level| level.parse().ok())
                .collect()
        })
        .collect()
}

fn is_safe(levels: &[i32]) -> bool {
    let increasing = levels
        .windows(2)
        .all(|w| w[1] - w[0] >= 1 && w[1] - w[0] <= 3);
    let decreasing = levels
        .windows(2)
        .all(|w| w[0] - w[1] >= 1 && w[0] - w[1] <= 3);
    increasing || decreasing
}

fn is_safe_single(levels: &[i32]) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for window in levels.windows(2) {
        let diff = window[1] - window[0];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        increasing &= diff > 0;
        decreasing &= diff < 0;
    }

    increasing || decreasing
}

fn is_safe_with_dampener(levels: &[i32]) -> bool {
    for i in 0..levels.len() {
        let mut modified_levels = levels.to_vec();
        modified_levels.remove(i);

        if is_safe(&modified_levels) {
            return true;
        }
    }
    false
}

#[aoc(day2, part1)]
pub fn part1(input: &[Vec<i32>]) -> u32 {
    input.iter().filter(|report| is_safe(report)).count() as u32
}

#[aoc(day2, part1, single)]
pub fn part1_single(input: &[Vec<i32>]) -> u32 {
    input.iter().filter(|report| is_safe_single(report)).count() as u32
}

#[aoc(day2, part2)]
pub fn part2(input: &[Vec<i32>]) -> u32 {
    input
        .iter()
        .filter(|report| is_safe_with_dampener(report))
        .count() as u32
}
