use common::*;

include_input!(INPUT);

fn letter_mask(letter: char) -> u32 {
    let number = letter as u8 - b'a';
    1 << number
}

fn parse_input(raw: &str) -> Vec<&str> {
    raw.lines().collect()
}

fn find_marker(data: &str, length: usize) -> usize {
    let (marker_index, _) = (0..data.len() - length)
        .enumerate()
        .find(|(_, start_index)| {
            let mut final_mask = 0;

            data[*start_index..*start_index + length]
                .chars()
                .map(letter_mask)
                .for_each(|mask| final_mask |= mask);

            final_mask.count_ones() as usize == length
        })
        .unwrap();

    marker_index + length
}

fn find_marker_short(data: &str) -> usize {
    find_marker(data, 4)
}

fn find_marker_long(data: &str) -> usize {
    find_marker(data, 14)
}

fn part_one(data: &[&str]) -> usize {
    data.iter().map(|line| find_marker_short(line)).sum()
}

fn part_two(data: &[&str]) -> usize {
    data.iter().map(|line| find_marker_long(line)).sum()
}

fn main() {
    let data = parse_input(INPUT);

    advent_solution(2022, 6, part_one(&data), part_two(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letter_mask_a() {
        assert_eq!(letter_mask('a'), 1);
    }

    #[test]
    fn letter_mask_b() {
        assert_eq!(letter_mask('b'), 2);
    }

    #[test]
    fn letter_mask_z() {
        assert_eq!(letter_mask('z'), 1 << 25);
    }

    #[test]
    fn example_1_0() {
        assert_eq!(find_marker_short("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    }

    #[test]
    fn example_1_1() {
        assert_eq!(find_marker_short("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    }

    #[test]
    fn example_1_2() {
        assert_eq!(find_marker_short("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    }

    #[test]
    fn example_1_3() {
        assert_eq!(find_marker_short("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn example_2_0() {
        assert_eq!(find_marker_long("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    }

    #[test]
    fn example_2_1() {
        assert_eq!(find_marker_long("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    }

    #[test]
    fn example_2_2() {
        assert_eq!(find_marker_long("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    }

    #[test]
    fn example_2_3() {
        assert_eq!(find_marker_long("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    }

    #[test]
    fn example_2_4() {
        assert_eq!(find_marker_long("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }

    #[test]
    fn part_one_final() {
        let data = parse_input(INPUT);
        assert_eq!(part_one(&data), 1140);
    }

    #[test]
    fn part_two_final() {
        let data = parse_input(INPUT);
        assert_eq!(part_two(&data), 3495);
    }
}
