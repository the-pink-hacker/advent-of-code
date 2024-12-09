use std::fmt::Display;

pub fn advent_solution(day: u8, part_one: impl Display, part_two: impl Display) {
    println!(
        "=== Day {} ===\n\nPart One:\n{}\n\nPart Two:\n{}",
        day, part_one, part_two
    );
}

#[macro_export]
macro_rules! include_input {
    ($var: ident) => {
        const $var: &str = include_str!("../input");
    };
}
