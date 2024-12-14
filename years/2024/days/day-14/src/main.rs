use std::{cmp::Ordering, collections::HashSet};

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

    fn print(&self, seconds: u32) {
        let robots = self
            .robots
            .iter()
            .map(|robot| self.calculate_final_position(robot, seconds))
            .collect::<HashSet<_>>();

        let deviation = self.x_deviation(seconds);

        if deviation > 19 {
            return;
        }

        println!("\n\n====================================================================\n\n");

        (0..self.height).for_each(|y| {
            let line = (0..self.width)
                .map(|x| match robots.get(&(x, y)) {
                    Some(_) => '#',
                    None => ' ',
                })
                .collect::<String>();

            println!("{}", line);
        });

        println!("Seconds: {}", seconds);
        println!("Deviation: {}", deviation);

        let mut x = String::new();
        std::io::stdin().read_line(&mut x).unwrap();
    }

    fn x_deviation(&self, seconds: u32) -> u8 {
        let positions = self
            .robots
            .iter()
            .map(|robot| self.calculate_final_position(robot, seconds).0 as usize)
            .collect::<Vec<_>>();
        let length = positions.len();

        let mean = positions.iter().sum::<usize>() / length;
        let variance = positions
            .into_iter()
            .map(|position| (position - mean).pow(2))
            .sum::<usize>()
            / length;

        variance.isqrt() as u8
    }

    fn part_one(&self) -> u32 {
        self.process(100).into_iter().product()
    }

    fn part_two(&self) {
        (0..u32::MAX).for_each(|i| self.print(i))
    }
}

fn parse_input(raw: &str) -> Vec<Robot> {
    raw.lines().map(Robot::from_raw).collect()
}

fn main() {
    let room = Room::new_big(parse_input(INPUT));
    room.part_two();
    advent_solution(2024, 14, room.part_one(), "");
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
}
