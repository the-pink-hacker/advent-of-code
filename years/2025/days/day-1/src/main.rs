use std::{
    ops::{Add, AddAssign, Sub},
    str::FromStr,
};

use anyhow::{Context, anyhow};
use common::*;

include_input!(INPUT);

const START_ROTATION: u16 = 50;
const FULL_ROTATION: u16 = 100;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct RotationValue(u16);

impl RotationValue {
    const fn add_rotation(self, rhs: Self) -> (Self, u16) {
        let sum = self.0 + rhs.0;

        (Self(sum % FULL_ROTATION), sum / FULL_ROTATION)
    }

    fn sub_rotation(self, rhs: Self) -> (Self, u16) {
        let diff = self.0.cast_signed() - rhs.0.cast_signed();
        let first_zero = if self.0 == 0 { 0 } else { 100 };
        let zeros = (first_zero - diff).cast_unsigned() / FULL_ROTATION;

        (
            Self(diff.rem_euclid(FULL_ROTATION.cast_signed()).cast_unsigned()),
            zeros,
        )
    }
}

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
        let first = s.chars().next().context("Rotation is empty.")?;
        let distance_raw = &s[1..];
        let distance = distance_raw
            .parse::<u16>()
            .with_context(|| format!("Failed to parse rotation distance: {distance_raw}"))?;
        let distance_clamped = distance % FULL_ROTATION;

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
        self.add_rotation(rhs).0
    }
}

impl Sub for RotationValue {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self.sub_rotation(rhs).0
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
    let mut password = 0;
    let mut current_rotation = RotationValue::default();

    for &rotation in rotations {
        let (new_rotation, zeros) = match rotation {
            Rotation::Left(value) => current_rotation.sub_rotation(value),
            Rotation::Right(value) => current_rotation.add_rotation(value),
        };

        current_rotation = new_rotation;
        password += zeros as usize;
    }

    password
}

fn main() -> anyhow::Result<()> {
    let rotations = parse_input(INPUT)?;

    advent_solution(2024, 9, part_one(&rotations), part_two(&rotations));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

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
        let rotations = parse_input(EXAMPLE).unwrap();
        assert_eq!(part_one(&rotations), 3);
    }

    #[test]
    fn part_one_final() {
        let rotations = parse_input(INPUT).unwrap();
        assert_eq!(part_one(&rotations), 964);
    }

    #[test]
    fn rotation_add_zeros() {
        let rotation = RotationValue(0);
        let other = RotationValue(300);
        let (sum, zeros) = rotation.add_rotation(other);
        assert_eq!(sum, RotationValue(0));
        assert_eq!(zeros, 3);
    }

    #[test]
    fn rotation_sub_zeros() {
        let rotation = RotationValue(10);
        let other = RotationValue(10);
        let (sum, zeros) = rotation.sub_rotation(other);
        assert_eq!(sum, RotationValue(0));
        assert_eq!(zeros, 1);
    }

    #[test]
    fn rotation_sub_zeros_overflow() {
        let rotation = RotationValue(0);
        let other = RotationValue(300);
        let (sum, zeros) = rotation.sub_rotation(other);
        assert_eq!(sum, RotationValue(0));
        assert_eq!(zeros, 3);
    }

    #[test]
    fn part_two_example() {
        let rotations = parse_input(EXAMPLE).unwrap();
        assert_eq!(part_two(&rotations), 6);
    }

    #[test]
    fn part_two_example_0() {
        let rotation = RotationValue(50);
        let other = RotationValue(68);
        let (sum, zeros) = rotation.sub_rotation(other);
        assert_eq!(sum, RotationValue(82));
        assert_eq!(zeros, 1);
    }

    #[test]
    fn part_two_example_1() {
        let rotation = RotationValue(82);
        let other = RotationValue(30);
        let (sum, zeros) = rotation.sub_rotation(other);
        assert_eq!(sum, RotationValue(52));
        assert_eq!(zeros, 0);
    }

    #[test]
    fn part_two_example_2() {
        let rotation = RotationValue(52);
        let other = RotationValue(48);
        let (sum, zeros) = rotation.add_rotation(other);
        assert_eq!(sum, RotationValue(0));
        assert_eq!(zeros, 1);
    }

    #[test]
    fn part_two_example_3() {
        let rotation = RotationValue(0);
        let other = RotationValue(5);
        let (sum, zeros) = rotation.sub_rotation(other);
        assert_eq!(sum, RotationValue(95));
        assert_eq!(zeros, 0);
    }

    #[test]
    fn part_two_example_4() {
        let rotation = RotationValue(95);
        let other = RotationValue(60);
        let (sum, zeros) = rotation.add_rotation(other);
        assert_eq!(sum, RotationValue(55));
        assert_eq!(zeros, 1);
    }

    #[test]
    fn part_two_example_5() {
        let rotation = RotationValue(55);
        let other = RotationValue(55);
        let (sum, zeros) = rotation.sub_rotation(other);
        assert_eq!(sum, RotationValue(0));
        assert_eq!(zeros, 1);
    }

    #[test]
    fn part_two_example_6() {
        let rotation = RotationValue(0);
        let other = RotationValue(1);
        let (sum, zeros) = rotation.sub_rotation(other);
        assert_eq!(sum, RotationValue(99));
        assert_eq!(zeros, 0);
    }

    #[test]
    fn part_two_example_7() {
        let rotation = RotationValue(99);
        let other = RotationValue(99);
        let (sum, zeros) = rotation.sub_rotation(other);
        assert_eq!(sum, RotationValue(0));
        assert_eq!(zeros, 1);
    }

    #[test]
    fn part_two_example_8() {
        let rotation = RotationValue(0);
        let other = RotationValue(14);
        let (sum, zeros) = rotation.add_rotation(other);
        assert_eq!(sum, RotationValue(14));
        assert_eq!(zeros, 0);
    }

    #[test]
    fn part_two_example_9() {
        let rotation = RotationValue(14);
        let other = RotationValue(82);
        let (sum, zeros) = rotation.sub_rotation(other);
        assert_eq!(sum, RotationValue(32));
        assert_eq!(zeros, 1);
    }

    #[test]
    fn part_two_example_10() {
        let rotation = RotationValue(50);
        let other = RotationValue(1000);
        let (sum, zeros) = rotation.add_rotation(other);
        assert_eq!(sum, RotationValue(50));
        assert_eq!(zeros, 10);
    }

    #[test]
    fn part_two_final() {
        let rotations = parse_input(INPUT).unwrap();
        let answer = part_two(&rotations);
        assert_eq!(answer, 5872); // How the *FUCKK* is this so off?!?!?!
    }
}
