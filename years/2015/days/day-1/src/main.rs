use common::*;

include_input!(INPUT);

fn step(value: char, floor: &mut i32) {
    match value {
        ')' => *floor -= 1,
        '(' => *floor += 1,
        _ => (),
    }
}

fn part_one(input: &str) -> i32 {
    let mut floor = 0;

    input.chars().for_each(|value| step(value, &mut floor));

    floor
}

fn part_two(input: &str) -> usize {
    let mut floor = 0;

    for (i, value) in input.chars().enumerate() {
        step(value, &mut floor);

        if floor == -1 {
            return i + 1;
        }
    }

    panic!("Never entered basment; current floor: {}", floor);
}

fn main() {
    advent_solution(2015, 1, part_one(INPUT), part_two(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_final() {
        assert_eq!(part_one(INPUT), 74);
    }

    #[test]
    fn part_two_final() {
        assert_eq!(part_two(INPUT), 1795);
    }
}
