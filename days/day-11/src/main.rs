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

    fn blink(stone: usize, stones: &mut Vec<usize>) {
        match stone {
            0 => stones.push(1),
            stone if (stone.ilog10() + 1) % 2 == 0 => {
                let (left, right) = split_number(stone);
                stones.push(left);
                stones.push(right);
            }
            _ => stones.push(stone * 2024),
        }
    }

    fn part_one(&mut self) -> usize {
        for _ in 0..25 {
            let mut new_stones = Vec::with_capacity(self.stones.len() * 2);

            for stone in &self.stones {
                Self::blink(*stone, &mut new_stones);
            }

            self.stones = new_stones;
        }

        self.stones.len()
    }
}

fn main() {
    let mut stones = Stones::new(INPUT);

    advent_solution(11, stones.part_one(), "");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_ONE: &str = "125 17";

    #[test]
    fn example_1() {
        let mut stones = Stones::new(EXAMPLE_ONE);
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
        let mut stones = Stones::new(INPUT);
        assert_eq!(stones.part_one(), 194557);
    }
}
