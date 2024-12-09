use std::{iter::Peekable, str::Chars};

use common::*;

include_input!(INPUT);

// Part One horrors we don't speak of
//
//use pcre2::bytes::Regex;
//
//const REGEX: &str = r"(?<=mul\()[0-9]+,[0-9]+(?=\))";
//
//fn main() {
//    let x = Regex::new(REGEX).unwrap();
//    let y = x.find_iter(INPUT.as_bytes());
//
//    let mut output = 0;
//
//    for z in y {
//        if let Ok(z) = z {
//            let bytes = z.as_bytes();
//            let x = String::from_utf8(bytes.to_vec()).unwrap();
//            dbg!(&x);
//            let (left, right) = x.split_once(',').unwrap();
//            let left = left.parse::<u32>().unwrap();
//            let right = right.parse::<u32>().unwrap();
//
//            output += left * right;
//        }
//    }
//
//    dbg!(output);
//}

enum ProgramFunction {
    Do,
    Dont,
    Multiply(u32, u32),
}

impl ProgramFunction {
    fn multiply(&self) -> Option<u32> {
        match self {
            Self::Multiply(a, b) => Some(a * b),
            _ => None,
        }
    }
}

struct Parser<'a> {
    view: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    fn new(buffer: &'a str) -> Self {
        Self {
            view: buffer.chars().peekable(),
        }
    }

    fn take_char(&mut self, value: char) -> bool {
        if let Some(found_char) = self.view.peek() {
            if *found_char == value {
                self.view.next();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn take_string(&mut self, value: &str) -> (bool, usize) {
        let mut consumed = 0;

        for value_char in value.chars() {
            if self.take_char(value_char) {
                consumed += 1;
            } else {
                return (false, consumed);
            }
        }

        (true, consumed)
    }

    fn take_number(&mut self) -> Option<u32> {
        let mut number_string = String::new();

        while let Some(found_char) = self.view.next_if(char::is_ascii_digit) {
            number_string.push(found_char);
        }

        number_string.parse().ok()
    }

    fn seek_function(&mut self) -> Option<ProgramFunction> {
        while let Some(found_char) = self.view.next() {
            match found_char {
                'd' => {
                    let (found, consumed) = self.take_string("o()");

                    if found {
                        return Some(ProgramFunction::Do);
                    } else if consumed == 1 && self.take_string("n't()").0 {
                        return Some(ProgramFunction::Dont);
                    }
                }
                'm' => {
                    if self.take_string("ul(").0 {
                        if let Some(number_a) = self.take_number() {
                            if self.take_char(',') {
                                if let Some(number_b) = self.take_number() {
                                    if self.take_char(')') {
                                        return Some(ProgramFunction::Multiply(number_a, number_b));
                                    }
                                }
                            }
                        }
                    }
                }
                _ => (),
            }
        }

        None
    }

    fn functions(mut self) -> Vec<ProgramFunction> {
        let mut functions = Vec::new();

        while let Some(function) = self.seek_function() {
            functions.push(function);
        }

        functions
    }
}

fn part_one(functions: &[ProgramFunction]) -> u32 {
    functions.iter().filter_map(ProgramFunction::multiply).sum()
}

fn part_two(functions: &[ProgramFunction]) -> u32 {
    let mut enabled = true;
    let mut output = 0;

    for function in functions {
        match function {
            ProgramFunction::Do => enabled = true,
            ProgramFunction::Dont => enabled = false,
            ProgramFunction::Multiply(number_a, number_b) => {
                if enabled {
                    output += number_a * number_b;
                }
            }
        }
    }

    output
}

fn main() {
    let parser = Parser::new(INPUT);
    let functions = parser.functions();

    advent_solution(3, part_one(&functions), part_two(&functions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let parser =
            Parser::new("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(part_one(&parser.functions()), 161);
    }

    #[test]
    fn part_two_example() {
        let parser = Parser::new(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );
        assert_eq!(part_two(&parser.functions()), 48);
    }

    #[test]
    fn part_one_final() {
        let parser = Parser::new(INPUT);
        assert_eq!(part_one(&parser.functions()), 179571322);
    }

    #[test]
    fn part_two_final() {
        let parser = Parser::new(INPUT);
        assert_eq!(part_two(&parser.functions()), 103811193);
    }
}
