use std::collections::HashMap;

use common::*;

include_input!(INPUT);

fn split_number(value: usize) -> (usize, usize) {
    let digits = value.ilog10() + 1;
    let left_scale = 10u32.pow(digits / 2) as usize;
    let left = value / left_scale;
    let right = value - (left * left_scale);
    (left, right)
}

struct Stones {
    stones: Vec<usize>,
}

impl Stones {
    fn new(raw: &str) -> Self {
        let stones = raw
            .split_whitespace()
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        Self { stones }
    }

    fn blink(stone: usize, mut depth: u8, cache: &mut HashMap<(u8, usize), usize>) -> usize {
        if depth == 0 {
            return 1;
        }

        depth -= 1;

        if let Some(value) = cache.get(&(depth, stone)) {
            return *value;
        }

        let amount = match stone {
            0 => Self::blink(1, depth, cache),
            stone if (stone.ilog10() + 1) % 2 == 0 => {
                let (left, right) = split_number(stone);
                Self::blink(left, depth, cache) + Self::blink(right, depth, cache)
            }
            _ => Self::blink(stone * 2024, depth, cache),
        };

        cache.insert((depth, stone), amount);
        amount
    }

    fn blink_many(&self, times: u8) -> usize {
        let mut cache = HashMap::new();

        self.stones
            .iter()
            .map(|stone| Self::blink(*stone, times, &mut cache))
            .sum()
    }

    fn part_one(&self) -> usize {
        self.blink_many(25)
    }

    fn part_two(&self) -> usize {
        self.blink_many(75)
    }
}

fn main() {
    let stones = Stones::new(INPUT);

    advent_solution(11, stones.part_one(), stones.part_two());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_ONE: &str = "125 17";

    #[test]
    fn example_1() {
        let stones = Stones::new(EXAMPLE_ONE);
        assert_eq!(stones.part_one(), 55312);
    }

    #[test]
    fn split_number() {
        let (left, right) = super::split_number(123456);
        assert_eq!(left, 123);
        assert_eq!(right, 456);
    }

    #[test]
    fn part_one_final() {
        let stones = Stones::new(INPUT);
        assert_eq!(stones.part_one(), 194557);
    }

    #[test]
    fn part_two_final() {
        let stones = Stones::new(INPUT);
        assert_eq!(stones.part_two(), 231532558973909);
    }
}
