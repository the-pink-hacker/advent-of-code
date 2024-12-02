#![feature(iter_array_chunks)]

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

fn main() {
    let (mut left, mut right) = split_data(INPUT);

    left.sort();
    right.sort();

    let total_distance = distance(left, right);

    println!("Total Distance: {}", total_distance);
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
}
