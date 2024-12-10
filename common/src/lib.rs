#![feature(step_trait)]

use std::{fmt::Display, iter::Step};

use num_traits::PrimInt;

pub fn advent_solution(day: u8, part_one: impl Display, part_two: impl Display) {
    println!(
        "=== Day {} ===\n\nPart One:\n{}\n\nPart Two:\n{}",
        day, part_one, part_two
    );
}

pub fn width_height_start_2d_iter<W, H>(
    start_x: W,
    start_y: H,
    width: W,
    height: H,
) -> impl Iterator<Item = (W, H)>
where
    W: PrimInt + Step,
    H: PrimInt + Step,
{
    (start_y..height).flat_map(move |y| (start_x..width).map(move |x| (x, y)))
}

pub fn width_height_2d_iter<W, H>(width: W, height: H) -> impl Iterator<Item = (W, H)>
where
    W: PrimInt + Step,
    H: PrimInt + Step,
{
    width_height_start_2d_iter(W::zero(), H::zero(), width, height)
}

#[macro_export]
macro_rules! include_input {
    ($var: ident) => {
        const $var: &str = include_str!("../input");
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_2d_iter() {
        let mut grid = width_height_2d_iter(5, 2);

        assert_eq!(grid.next(), Some((0, 0)));
        assert_eq!(grid.next(), Some((1, 0)));
        assert_eq!(grid.next(), Some((2, 0)));
        assert_eq!(grid.next(), Some((3, 0)));
        assert_eq!(grid.next(), Some((4, 0)));
        assert_eq!(grid.next(), Some((0, 1)));
        assert_eq!(grid.next(), Some((1, 1)));
        assert_eq!(grid.next(), Some((2, 1)));
        assert_eq!(grid.next(), Some((3, 1)));
        assert_eq!(grid.next(), Some((4, 1)));
        assert_eq!(grid.next(), None);
    }
}
