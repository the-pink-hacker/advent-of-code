use common::*;

const CONVERSION_OFFSET: u64 = 10_000_000_000_000;

include_input!(INPUT);

#[derive(Debug, Eq, PartialEq)]
struct Game {
    button_a: (u64, u64),
    button_b: (u64, u64),
    prize: (u64, u64),
}

impl Game {
    fn new(raw: &str) -> Self {
        let mut lines = raw.lines();
        let button_a = Self::parse_button(lines.next().unwrap());
        let button_b = Self::parse_button(lines.next().unwrap());

        let (_, right) = lines.next().unwrap().split_once('=').unwrap();
        let (x, y) = right.split_once(',').unwrap();
        let (_, y) = y.split_once('=').unwrap();
        let prize = (x.parse().unwrap(), y.parse().unwrap());

        Self {
            button_a,
            button_b,
            prize,
        }
    }

    fn parse_button(raw: &str) -> (u64, u64) {
        let (_, right) = raw.split_once('+').unwrap();
        let (x, y) = right.split_once(',').unwrap();
        let (_, y) = y.split_once('+').unwrap();
        (x.parse().unwrap(), y.parse().unwrap())
    }

    fn min_tokens(&self) -> Option<u64> {
        let (button_a_x, button_a_y) = self.button_a;
        let (button_b_x, button_b_y) = self.button_b;
        let (prize_x, prize_y) = self.prize;

        let numerator = (prize_x * button_b_y) as i64 - (prize_y * button_b_x) as i64;
        let denominator = (button_a_x * button_b_y) as i64 - (button_a_y * button_b_x) as i64;

        if numerator % denominator == 0 {
            let button_a = (numerator / denominator) as u64;
            let button_b = (prize_x - (button_a_x * button_a)) / button_b_x;

            // WHY????????????
            if button_a * button_a_x + button_b * button_b_x != prize_x {
                return None;
            }

            Some(button_a * 3 + button_b)
        } else {
            None
        }
    }

    fn correct_conversion(&mut self) {
        self.prize.0 += CONVERSION_OFFSET;
        self.prize.1 += CONVERSION_OFFSET;
    }
}

fn parse_input(raw: &str) -> Vec<Game> {
    raw.split("\n\n").map(Game::new).collect()
}

fn part_one(games: &[Game]) -> u64 {
    games.iter().filter_map(Game::min_tokens).sum()
}

fn part_two(mut games: Vec<Game>) -> u64 {
    games.iter_mut().for_each(Game::correct_conversion);
    games.iter().filter_map(Game::min_tokens).sum()
}

fn main() {
    let games = parse_input(INPUT);

    advent_solution(2024, 13, part_one(&games), part_two(games));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_ONE: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn parse() {
        let games = parse_input(EXAMPLE_ONE);
        let expected = vec![
            Game {
                button_a: (94, 34),
                button_b: (22, 67),
                prize: (8400, 5400),
            },
            Game {
                button_a: (26, 66),
                button_b: (67, 21),
                prize: (12748, 12176),
            },
            Game {
                button_a: (17, 86),
                button_b: (84, 37),
                prize: (7870, 6450),
            },
            Game {
                button_a: (69, 23),
                button_b: (27, 71),
                prize: (18641, 10279),
            },
        ];

        assert_eq!(games, expected);
    }

    #[test]
    fn example_1() {
        let games = parse_input(EXAMPLE_ONE);
        assert_eq!(part_one(&games), 480);
    }

    #[test]
    fn part_one_final() {
        let games = parse_input(INPUT);
        assert_eq!(part_one(&games), 28059);
    }

    #[test]
    fn part_two_final() {
        let games = parse_input(INPUT);
        assert_eq!(part_two(games), 102255878088512);
    }
}
