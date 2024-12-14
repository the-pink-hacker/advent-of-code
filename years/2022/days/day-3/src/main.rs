#![feature(iter_array_chunks)]

use common::*;

include_input!(INPUT);

fn char_bitmask(value: char) -> u64 {
    let place = match value {
        'A'..='Z' => 26 + value as u8 - b'A',
        'a'..='z' => value as u8 - b'a',
        _ => panic!("Char unsupported: {}", value),
    } + 1;

    1 << place
}

fn string_bitmask(value: &str) -> u64 {
    let mut output = 0;

    value
        .chars()
        .map(char_bitmask)
        .for_each(|bitmask| output |= bitmask);

    output
}

fn bitmask_inverse(value: u64) -> u8 {
    value.ilog2() as u8
}

fn split_half(value: &str) -> (&str, &str) {
    value.split_at(value.len() / 2)
}

fn parse_input(raw: &str) -> Vec<&str> {
    raw.lines().collect()
}

fn part_one(sacks: &[&str]) -> u32 {
    sacks
        .iter()
        .cloned()
        .map(split_half)
        .map(|(left, right)| string_bitmask(left) & string_bitmask(right))
        .map(bitmask_inverse)
        .map(u32::from)
        .sum()
}

fn part_two(sacks: &[&str]) -> u32 {
    sacks
        .iter()
        .array_chunks::<3>()
        .map(|[a, b, c]| string_bitmask(a) & string_bitmask(b) & string_bitmask(c))
        .map(bitmask_inverse)
        .map(u32::from)
        .sum()
}

fn main() {
    let sacks = parse_input(INPUT);

    advent_solution(2022, 3, part_one(&sacks), part_two(&sacks));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_ONE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    const EXAMPLE_TWO: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg";

    const EXAMPLE_THREE: &str = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn split() {
        let (left, right) = split_half("vJrwpWtwJgWrhcsFMMfFFhFp");
        assert_eq!(left, "vJrwpWtwJgWr");
        assert_eq!(right, "hcsFMMfFFhFp");
    }

    #[test]
    fn bitmask_inverse_a() {
        let bitmask = char_bitmask('a');
        assert_eq!(bitmask_inverse(bitmask), 1);
    }

    #[test]
    fn bitmask_inverse_b() {
        let bitmask = char_bitmask('b');
        assert_eq!(bitmask_inverse(bitmask), 2);
    }

    #[test]
    fn bitmask_inverse_z() {
        let bitmask = char_bitmask('Z');
        assert_eq!(bitmask_inverse(bitmask), 52);
    }

    #[test]
    fn example_1() {
        let sacks = parse_input(EXAMPLE_ONE);
        assert_eq!(part_one(&sacks), 157);
    }

    #[test]
    fn example_2() {
        let sacks = parse_input(EXAMPLE_TWO);
        assert_eq!(part_two(&sacks), 18);
    }

    #[test]
    fn example_3() {
        let sacks = parse_input(EXAMPLE_THREE);
        assert_eq!(part_two(&sacks), 52);
    }

    #[test]
    fn part_one_final() {
        let sacks = parse_input(INPUT);
        assert_eq!(part_one(&sacks), 7908);
    }

    #[test]
    fn part_two_final() {
        let sacks = parse_input(INPUT);
        assert_eq!(part_two(&sacks), 2838);
    }
}
