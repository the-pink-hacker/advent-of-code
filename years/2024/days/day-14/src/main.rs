use std::{cmp::Ordering, collections::HashMap};

use common::*;

include_input!(INPUT);

#[derive(Debug, PartialEq, Eq)]
struct Robot {
    position: (u8, u8),
    velocity: (i8, i8),
}

impl Robot {
    fn from_raw(raw: &str) -> Self {
        let (x, right) = raw[2..].split_once(',').unwrap();
        let (y, right) = right.split_once(' ').unwrap();
        let (velocity_x, velocity_y) = right[2..].split_once(',').unwrap();

        let position = (x.parse().unwrap(), y.parse().unwrap());
        let velocity = (velocity_x.parse().unwrap(), velocity_y.parse().unwrap());

        Self { position, velocity }
    }
}

struct Room {
    width: u8,
    height: u8,
    width_middle: u8,
    height_middle: u8,
    robots: Vec<Robot>,
}

impl Room {
    fn new_big() -> Self {
        Self::from_size(101, 103)
    }

    fn new_small() -> Self {
        Self::from_size(11, 7)
    }

    fn from_size(width: u8, height: u8) -> Self {
        let width_middle = width / 2;
        let height_middle = height / 2;
        let robots = Vec::new();

        Self {
            width,
            height,
            width_middle,
            height_middle,
            robots,
        }
    }

    fn calculate_final_position(&self, robot: &Robot, seconds: u32) -> (u8, u8) {
        let (x, y) = robot.position;
        let (velocity_x, velocity_y) = robot.velocity;

        let total_x = x as i32 + (velocity_x as i32 * seconds as i32);
        let total_y = y as i32 + (velocity_y as i32 * seconds as i32);
        let wrapped_x = (total_x.rem_euclid(self.width as i32)) as u8;
        let wrapped_y = (total_y.rem_euclid(self.height as i32)) as u8;

        (wrapped_x, wrapped_y)
    }

    fn quad_index(&self, position: (u8, u8)) -> Option<usize> {
        let (x, y) = position;

        match x.cmp(&self.width_middle) {
            Ordering::Less => match y.cmp(&self.height_middle) {
                Ordering::Less => Some(1),
                Ordering::Equal => None,
                Ordering::Greater => Some(2),
            },
            Ordering::Equal => None,
            Ordering::Greater => match y.cmp(&self.height_middle) {
                Ordering::Less => Some(0),
                Ordering::Equal => None,
                Ordering::Greater => Some(3),
            },
        }
    }

    fn process(&self, robots: &[Robot], seconds: u32) -> [u32; 4] {
        let mut quads = [0; 4];

        robots
            .iter()
            .map(|robot| self.calculate_final_position(robot, seconds))
            .filter_map(|position| self.quad_index(position))
            .for_each(|quad_index| quads[quad_index] += 1);

        quads
    }

    fn part_one(&self, robots: &[Robot]) -> u32 {
        self.process(robots, 100).into_iter().product()
    }
}

fn parse_input(raw: &str) -> Vec<Robot> {
    raw.lines().map(Robot::from_raw).collect()
}

fn main() {
    let robots = parse_input(INPUT);
    let room = Room::new_big();
    advent_solution(2024, 14, room.part_one(&robots), "");
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_ONE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn parse_robot() {
        let robot = Robot::from_raw("p=40,73 v=-96,64");
        let expected = Robot {
            position: (40, 73),
            velocity: (-96, 64),
        };
        assert_eq!(robot, expected);
    }

    #[test]
    fn example_1() {
        let robots = parse_input(EXAMPLE_ONE);
        assert_eq!(Room::new_small().part_one(&robots), 12);
    }

    #[test]
    fn part_one_final() {
        let robots = parse_input(INPUT);
        assert_eq!(Room::new_big().part_one(&robots), 218295000);
    }
}
