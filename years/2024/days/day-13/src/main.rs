use common::*;

const MAX_PRESSES: u8 = 100;

include_input!(INPUT);

#[derive(Debug, Eq, PartialEq)]
struct Game {
    button_a: (u32, u32),
    button_b: (u32, u32),
    prize: (u32, u32),
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

    fn parse_button(raw: &str) -> (u32, u32) {
        let (_, right) = raw.split_once('+').unwrap();
        let (x, y) = right.split_once(',').unwrap();
        let (_, y) = y.split_once('+').unwrap();
        (x.parse().unwrap(), y.parse().unwrap())
    }

    fn min_tokens(&self) -> Option<u32> {
        let (button_a_x, button_a_y) = self.button_a;
        let (button_b_x, button_b_y) = self.button_b;
        let (prize_x, prize_y) = self.prize;

        for a_presses in 0..MAX_PRESSES {
            let a_x_amount = button_a_x * a_presses as u32;
            let a_y_amount = button_a_y * a_presses as u32;

            if let Some(remaining_x) = prize_x.checked_sub(a_x_amount) {
                let b_presses = remaining_x / button_b_x;

                let b_x_amount = button_b_x * b_presses;
                let b_y_amount = button_b_y * b_presses;

                let x_amount = a_x_amount + b_x_amount;
                let y_amount = a_y_amount + b_y_amount;

                if x_amount == prize_x && y_amount == prize_y {
                    return Some(a_presses as u32 * 3 + b_presses);
                }
            }
        }

        None
    }
}

fn parse_input(raw: &str) -> Vec<Game> {
    raw.split("\n\n").map(Game::new).collect()
}

fn part_one(games: &[Game]) -> u32 {
    games.iter().filter_map(Game::min_tokens).sum()
}

fn main() {
    let games = parse_input(INPUT);

    advent_solution(2024, 13, part_one(&games), "");
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
}
