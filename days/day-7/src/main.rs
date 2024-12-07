const INPUT: &str = include_str!("../input");

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

impl Operator {
    fn apply(&self, left: usize, right: usize) -> usize {
        match self {
            Self::Add => left + right,
            Self::Multiply => left * right,
            Self::Concat => {
                let left_power = right.ilog10() + 1;
                let left = left * 10usize.pow(left_power);
                left + right
            }
        }
    }

    // Returns true if wraps
    fn advance(&mut self) -> bool {
        match self {
            Self::Add => {
                *self = Self::Multiply;
                false
            }
            Self::Multiply => {
                *self = Self::Add;
                true
            }
            Self::Concat => panic!("Concat unsupported"),
        }
    }

    // Returns true if wraps
    fn advance_concat(&mut self) -> bool {
        match self {
            Self::Add => {
                *self = Self::Multiply;
                false
            }
            Self::Multiply => {
                *self = Self::Concat;
                false
            }
            Self::Concat => {
                *self = Self::Add;
                true
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Equation {
    final_value: usize,
    values: Vec<usize>,
}

impl Equation {
    fn new(final_value: usize, values: Vec<usize>) -> Self {
        Self {
            final_value,
            values,
        }
    }

    fn parse(raw: &str) -> Self {
        let (left, right) = raw.split_once(':').unwrap();

        let final_value = left.parse().unwrap();
        let values = right
            .split_ascii_whitespace()
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        Self {
            final_value,
            values,
        }
    }

    fn eval(&self, operators: &[Operator]) -> usize {
        let mut values = self.values.clone();

        for (i, operator) in operators.iter().enumerate() {
            values[i + 1] = operator.apply(values[i], values[i + 1]);
        }

        *values.last().unwrap()
    }

    fn solve_operators(
        &self,
        advance: impl Fn(&mut Operator) -> bool,
        operator_number: u8,
    ) -> Option<usize> {
        let operators_length = self.values.len() - 1;
        let mut operators = vec![Operator::Add; operators_length];

        let loop_amount = (operator_number as u32).pow(operators_length as u32);

        for _ in 0..loop_amount {
            for operator in &mut operators {
                if !advance(operator) {
                    break;
                }
            }

            if self.eval(&operators) == self.final_value {
                return Some(self.final_value);
            }
        }

        None
    }

    fn part_one(&self) -> Option<usize> {
        self.solve_operators(Operator::advance, 2)
    }

    fn part_two(&self) -> Option<usize> {
        self.solve_operators(Operator::advance_concat, 3)
    }
}

fn parse_equations(raw: &str) -> Vec<Equation> {
    raw.lines().map(Equation::parse).collect()
}

fn part_one(equations: &[Equation]) -> usize {
    equations.iter().filter_map(Equation::part_one).sum()
}

fn part_two(equations: &[Equation]) -> usize {
    equations.iter().filter_map(Equation::part_two).sum()
}

fn main() {
    let equations = parse_equations(INPUT);

    println!("=== Day 7 ===");

    println!();
    println!("Part One:");
    println!("{}", part_one(&equations));

    println!();
    println!("Part Two:");
    println!("{}", part_two(&equations));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_ONE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn eval() {
        let equation = Equation::new(3267, vec![81, 40, 27]);

        let operations = &[Operator::Add, Operator::Multiply];

        assert_eq!(equation.eval(operations), 3267);
    }

    #[test]
    fn parse() {
        let raw = "1234: 1 2 3 4";
        let expected = Equation::new(1234, vec![1, 2, 3, 4]);

        assert_eq!(Equation::parse(raw), expected);
    }

    #[test]
    fn example_1_0() {
        let equation = Equation::new(190, vec![10, 19]);
        assert_eq!(equation.part_one(), Some(190));
    }

    #[test]
    fn example_1_1() {
        let equation = Equation::new(3267, vec![81, 40, 27]);
        assert_eq!(equation.part_one(), Some(3267));
    }

    #[test]
    fn example_1_2() {
        let equation = Equation::new(83, vec![17, 5]);
        assert_eq!(equation.part_one(), None);
    }

    #[test]
    fn example_1_3() {
        let equation = Equation::new(156, vec![15, 6]);
        assert_eq!(equation.part_one(), None);
    }

    #[test]
    fn example_1_4() {
        let equation = Equation::new(7290, vec![6, 8, 6, 15]);
        assert_eq!(equation.part_one(), None);
    }

    #[test]
    fn example_1_5() {
        let equation = Equation::new(161011, vec![16, 10, 13]);
        assert_eq!(equation.part_one(), None);
    }

    #[test]
    fn example_1_6() {
        let equation = Equation::new(192, vec![17, 8, 14]);
        assert_eq!(equation.part_one(), None);
    }

    #[test]
    fn example_1_7() {
        let equation = Equation::new(21037, vec![9, 7, 18, 13]);
        assert_eq!(equation.part_one(), None);
    }

    #[test]
    fn example_1_8() {
        let equation = Equation::new(292, vec![11, 6, 16, 20]);
        assert_eq!(equation.part_one(), Some(292));
    }

    #[test]
    fn example_1() {
        let equations = parse_equations(EXAMPLE_ONE);
        assert_eq!(part_one(&equations), 3749);
    }

    #[test]
    fn concat() {
        let concat = Operator::Concat.apply(123, 4567);
        assert_eq!(concat, 1234567);
    }

    #[test]
    fn example_2() {
        let equations = parse_equations(EXAMPLE_ONE);
        assert_eq!(part_two(&equations), 11387);
    }

    #[test]
    fn part_one_final() {
        let equations = parse_equations(INPUT);
        assert_eq!(part_one(&equations), 2654749936343);
    }

    #[test]
    fn part_two_final() {
        let equations = parse_equations(INPUT);
        assert_eq!(part_two(&equations), 124060392153684);
    }
}
