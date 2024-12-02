#![feature(iter_array_chunks)]

use std::collections::HashMap;

const INPUT: &str = include_str!("../input");

fn split_data(data: &str) -> (Vec<u32>, Vec<u32>) {
    data.split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .array_chunks::<2>()
        .map(|[left, right]| (left, right))
        .unzip::<u32, u32, _, _>()
}

fn distance(left: Vec<u32>, right: Vec<u32>) -> u32 {
    left.into_iter()
        .zip(right)
        .map(|(left_value, right_value)| {
            if left_value > right_value {
                left_value - right_value
            } else {
                right_value - left_value
            }
        })
        .sum()
}

#[derive(Debug, Default)]
struct Similarity {
    sum: u32,
    occurrences: u32,
}

impl Similarity {
    fn score(&self) -> u32 {
        self.sum * self.occurrences
    }
}

fn similarity_sum(similarity_map: &mut HashMap<u32, Similarity>, left: Vec<u32>) {
    left.into_iter()
        .for_each(|value| similarity_map.entry(value).or_default().sum += value);
}

fn similarity_occurrences(similarity_map: &mut HashMap<u32, Similarity>, right: Vec<u32>) {
    right
        .into_iter()
        .for_each(|value| similarity_map.entry(value).or_default().occurrences += 1);
}

fn calculate_similarity(similarity_map: HashMap<u32, Similarity>) -> u32 {
    similarity_map.values().map(Similarity::score).sum()
}

fn part_one(mut left: Vec<u32>, mut right: Vec<u32>) {
    left.sort();
    right.sort();

    let total_distance = distance(left, right);

    println!();
    println!("Part One");
    println!("Total Distance: {}", total_distance);
}

fn part_two(left: Vec<u32>, right: Vec<u32>) {
    let mut similarity_map = HashMap::<u32, Similarity>::new();

    similarity_sum(&mut similarity_map, left);
    similarity_occurrences(&mut similarity_map, right);

    let similarity = calculate_similarity(similarity_map);

    println!();
    println!("Part Two");
    println!("Similarity: {}", similarity);
}

fn main() {
    let (left, right) = split_data(INPUT);

    println!("=== Day One ===");

    part_one(left.clone(), right.clone());
    part_two(left, right);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "#;

    const EXAMPLE_SPLIT_LEFT: [u32; 6] = [3, 4, 2, 1, 3, 3];
    const EXAMPLE_SPLIT_RIGHT: [u32; 6] = [4, 3, 5, 3, 9, 3];

    #[test]
    fn example_split() {
        let (left, right) = split_data(EXAMPLE);

        assert_eq!(left, EXAMPLE_SPLIT_LEFT);
        assert_eq!(right, EXAMPLE_SPLIT_RIGHT);
    }

    #[test]
    fn example_distance() {
        let mut left = EXAMPLE_SPLIT_LEFT.to_vec();
        let mut right = EXAMPLE_SPLIT_RIGHT.to_vec();

        left.sort();
        right.sort();

        assert_eq!(distance(left, right), 11);
    }

    #[test]
    fn example_similarity() {
        let left = EXAMPLE_SPLIT_LEFT.to_vec();
        let right = EXAMPLE_SPLIT_RIGHT.to_vec();

        let mut similarity_map = HashMap::new();

        similarity_sum(&mut similarity_map, left);
        similarity_occurrences(&mut similarity_map, right);

        assert_eq!(calculate_similarity(similarity_map), 31);
    }
}
