use std::collections::{BTreeMap, HashMap};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (mut left, mut right) = (Vec::new(), Vec::new());

    for line in input.lines() {
        let (l, r) = line.split_once(' ').unwrap();

        left.push(l.parse().unwrap());
        right.push(r.trim_start().parse().unwrap());
    }

    left.sort_unstable();
    right.sort_unstable();

    (left, right)
}

#[aoc(day1, part1)]
pub fn solve_part1((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum()
}

#[aoc(day1, part2, btreemap)]
pub fn solve_part2_btreemap((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut freq = BTreeMap::new();

    right.iter().for_each(|r| *freq.entry(r).or_insert(0) += 1);

    left.iter().filter_map(|l| freq.get(l).map(|f| l * f)).sum()
}

#[aoc(day1, part2, naive)]
pub fn solve_part2_naive((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut similarity = 0;

    for l in left {
        similarity += l * right.iter().filter(|x| *x == l).count() as u32;
    }

    similarity
}

#[aoc(day1, part2, hashmap)]
pub fn solve_part2_hashmap((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut freq = HashMap::new();

    right.iter().for_each(|r| *freq.entry(r).or_insert(0) += 1);

    left.iter().filter_map(|l| freq.get(l).map(|f| l * f)).sum()
}

#[aoc(day1, part2, hashmap_alloc)]
pub fn solve_part2_hashmap_alloc((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut freq = HashMap::with_capacity(right.len());

    right.iter().for_each(|r| *freq.entry(r).or_insert(0) += 1);

    left.iter().filter_map(|l| freq.get(l).map(|f| l * f)).sum()
}
