use std::{
    ops::{Add, AddAssign, Sub},
    str::FromStr,
};

use anyhow::{Context, anyhow};
use common::*;

include_input!(INPUT);

const START_ROTATION: u8 = 50;
const FULL_ROTATION: u8 = 100;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct RotationValue(u8);

impl Default for RotationValue {
    fn default() -> Self {
        Self(START_ROTATION)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Rotation {
    Left(RotationValue),
    Right(RotationValue),
}

impl FromStr for Rotation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut chars = s.chars();
        let first = chars.next().context("Rotation is empty.")?;
        let distance_raw = chars.collect::<String>();
        let distance = distance_raw
            .parse::<u16>()
            .with_context(|| format!("Failed to parse rotation distance: {distance_raw}"))?;
        let distance_clamped = (distance % FULL_ROTATION as u16) as u8;

        match first {
            'R' => Ok(Self::Right(RotationValue(distance_clamped))),
            'L' => Ok(Self::Left(RotationValue(distance_clamped))),
            _ => Err(anyhow!("Unsupported rotation direction: {first}")),
        }
    }
}

impl Add for RotationValue {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self((self.0 + rhs.0) % FULL_ROTATION)
    }
}

impl Sub for RotationValue {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let (diff, overflowed) = self.0.overflowing_sub(rhs.0);

        if overflowed {
            Self(diff - (u8::MAX - FULL_ROTATION) - 1)
        } else {
            Self(diff)
        }
    }
}

impl Add<Rotation> for RotationValue {
    type Output = Self;

    fn add(self, rhs: Rotation) -> Self {
        match rhs {
            Rotation::Left(value) => self - value,
            Rotation::Right(value) => self + value,
        }
    }
}

impl AddAssign<Rotation> for RotationValue {
    fn add_assign(&mut self, rhs: Rotation) {
        *self = *self + rhs
    }
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Rotation>> {
    input.lines().map(Rotation::from_str).collect()
}

fn part_one(rotations: &[Rotation]) -> usize {
    let mut password = 0;
    let mut current_rotation = RotationValue::default();

    for &rotation in rotations {
        current_rotation += rotation;

        if current_rotation.0 == 0 {
            password += 1;
        }
    }

    password
}

fn part_two(rotations: &[Rotation]) -> usize {
    0
}

fn main() -> anyhow::Result<()> {
    let rotations = parse_input(INPUT)?;

    advent_solution(2024, 9, part_one(&rotations), part_two(&rotations));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotation_left_0() {
        let raw = "L0";
        let parsed = raw.parse::<Rotation>().unwrap();
        assert_eq!(parsed, Rotation::Left(RotationValue(0)));
    }

    #[test]
    fn rotation_left_50() {
        let raw = "L50";
        let parsed = raw.parse::<Rotation>().unwrap();
        assert_eq!(parsed, Rotation::Left(RotationValue(50)));
    }

    #[test]
    fn rotation_left_100() {
        let raw = "L100";
        let parsed = raw.parse::<Rotation>().unwrap();
        assert_eq!(parsed, Rotation::Left(RotationValue(0)));
    }

    #[test]
    fn rotation_right_0() {
        let raw = "R0";
        let parsed = raw.parse::<Rotation>().unwrap();
        assert_eq!(parsed, Rotation::Right(RotationValue(0)));
    }

    #[test]
    fn rotation_right_50() {
        let raw = "R50";
        let parsed = raw.parse::<Rotation>().unwrap();
        assert_eq!(parsed, Rotation::Right(RotationValue(50)));
    }

    #[test]
    fn rotation_right_100() {
        let raw = "R100";
        let parsed = raw.parse::<Rotation>().unwrap();
        assert_eq!(parsed, Rotation::Right(RotationValue(0)));
    }

    #[test]
    fn rotation_add_overflow() {
        let value = RotationValue(99);
        let rotation = Rotation::Right(RotationValue(99));
        assert_eq!(value + rotation, RotationValue(98));
    }

    #[test]
    fn rotation_add_underflow() {
        let value = RotationValue(0);
        let rotation = Rotation::Left(RotationValue(99));
        assert_eq!(value + rotation, RotationValue(1));
    }

    #[test]
    fn part_one_example() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        let rotations = parse_input(input).unwrap();
        assert_eq!(part_one(&rotations), 3);
    }

    #[test]
    fn part_one_test() {
        let rotations = parse_input(INPUT).unwrap();
        assert_eq!(part_one(&rotations), 964);
    }
}
