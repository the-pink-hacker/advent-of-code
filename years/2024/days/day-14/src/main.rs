use std::cmp::Ordering;

use common::*;

include_input!(INPUT);

fn standard_deviation(values: &[usize]) -> u8 {
    let mean = values.iter().sum::<usize>() / values.len();
    let variance = values
        .iter()
        .map(|value| value.abs_diff(mean).pow(2))
        .sum::<usize>()
        / values.len();

    variance.isqrt() as u8
}

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
    fn new_big(robots: Vec<Robot>) -> Self {
        Self::from_size(101, 103, robots)
    }

    #[allow(unused)]
    fn new_small(robots: Vec<Robot>) -> Self {
        Self::from_size(11, 7, robots)
    }

    fn from_size(width: u8, height: u8, robots: Vec<Robot>) -> Self {
        let width_middle = width / 2;
        let height_middle = height / 2;

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
        let wrapped_x = total_x.rem_euclid(self.width as i32) as u8;
        let wrapped_y = total_y.rem_euclid(self.height as i32) as u8;

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

    fn process(&self, seconds: u32) -> [u32; 4] {
        let mut quads = [0; 4];

        self.robots
            .iter()
            .map(|robot| self.calculate_final_position(robot, seconds))
            .filter_map(|position| self.quad_index(position))
            .for_each(|quad_index| quads[quad_index] += 1);

        quads
    }

    fn is_tree(&self, seconds: u32) -> bool {
        let (deviation_x, deviation_y) = self.deviations(seconds);

        deviation_x < 20 && deviation_y < 20
    }

    fn deviations(&self, seconds: u32) -> (u8, u8) {
        let (positions_x, positions_y) = self
            .robots
            .iter()
            .map(|robot| {
                let (x, y) = self.calculate_final_position(robot, seconds);
                (x as usize, y as usize)
            })
            .unzip::<_, _, Vec<_>, Vec<_>>();

        let deviation_x = standard_deviation(&positions_x);
        let deviation_y = standard_deviation(&positions_y);

        (deviation_x, deviation_y)
    }

    fn part_one(&self) -> u32 {
        self.process(100).into_iter().product()
    }

    fn part_two(&self) -> usize {
        (0..u32::MAX)
            .position(|seconds| self.is_tree(seconds))
            .unwrap()
    }
}

fn parse_input(raw: &str) -> Vec<Robot> {
    raw.lines().map(Robot::from_raw).collect()
}

fn main() {
    let room = Room::new_big(parse_input(INPUT));

    advent_solution(2024, 14, room.part_one(), room.part_two());
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
        let room = Room::new_small(parse_input(EXAMPLE_ONE));
        assert_eq!(room.part_one(), 12);
    }

    #[test]
    fn part_one_final() {
        let room = Room::new_big(parse_input(INPUT));
        assert_eq!(room.part_one(), 218295000);
    }

    #[test]
    fn part_two_final() {
        let room = Room::new_big(parse_input(INPUT));
        assert_eq!(room.part_two(), 6870);
    }
}
