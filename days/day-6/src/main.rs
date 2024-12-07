use std::{cell::OnceCell, collections::HashSet, sync::atomic::AtomicU32};

// If you see rayon you know this ain't good code
use rayon::prelude::*;

const INPUT: &str = include_str!("../input");

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn apply(&self, value: (u8, u8)) -> Option<(u8, u8)> {
        let (x, y) = value;

        match self {
            Self::Up => y.checked_sub(1).map(|y| (x, y)),
            Self::Down => Some((x, y + 1)),
            Self::Left => x.checked_sub(1).map(|x| (x, y)),
            Self::Right => Some((x + 1, y)),
        }
    }

    fn rotate(&mut self) {
        *self = match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
}

#[derive(Debug, Clone)]
struct LabMap {
    obstructions: HashSet<(u8, u8)>,
    guard_position: (u8, u8),
    guard_direction: Direction,
    width: u8,
    height: u8,
    visted: HashSet<(u8, u8)>,
}

impl LabMap {
    fn new(raw: &str) -> Self {
        let mut guard_position = OnceCell::new();
        let mut obstructions = HashSet::new();
        let mut height = 0;
        let mut width = 0;

        for (y, line) in raw.lines().enumerate() {
            height += 1;
            if y == 0 {
                width = line.len() as u8;
            }
            for (x, place) in line.chars().enumerate() {
                match place {
                    '#' => {
                        obstructions.insert((x as u8, y as u8));
                    }
                    '^' => guard_position.set((x as u8, y as u8)).unwrap(),
                    _ => (),
                }
            }
        }

        Self {
            obstructions,
            guard_position: guard_position.take().unwrap(),
            guard_direction: Direction::Up,
            visted: HashSet::new(),
            width,
            height,
        }
    }

    fn within_bounds(&self, position: (u8, u8)) -> Option<(u8, u8)> {
        let (x, y) = position;
        if x < self.width && y < self.height {
            Some(position)
        } else {
            None
        }
    }

    fn check_obstruction(&self, position: &(u8, u8)) -> bool {
        self.obstructions.contains(position)
    }

    fn travel(&mut self) -> bool {
        if let Some(new_position) = self
            .guard_direction
            .apply(self.guard_position)
            .and_then(|new_position| self.within_bounds(new_position))
        {
            // If surounded, will become infinite loop
            if self.check_obstruction(&new_position) {
                self.guard_direction.rotate();
            } else {
                self.visted.insert(self.guard_position);
                self.guard_position = new_position;
            }
            true
        } else {
            self.visted.insert(self.guard_position);
            false
        }
    }

    fn travel_all(&mut self) {
        while self.travel() {}
    }

    fn travel_all_capped(&mut self) -> bool {
        let max = 1_000_000;
        for i in 0..max {
            if !self.travel() {
                return true;
            }
        }

        false
    }

    fn amount_visted(&self) -> u32 {
        self.visted.len() as u32
    }

    fn part_one(mut self) -> u32 {
        self.travel_all();
        self.amount_visted()
    }

    fn part_two(&self) -> u32 {
        let mut positions = AtomicU32::new(0);

        for y in 0..self.height {
            (0..self.width).into_par_iter().for_each(|x| {
                if !self.obstructions.contains(&(x, y)) && self.guard_position != (x, y) {
                    let mut map_clone = self.clone();
                    map_clone.obstructions.insert((x, y));
                    if !map_clone.travel_all_capped() {
                        positions.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    }
                }
            })
        }

        *positions.get_mut()
    }
}

fn main() {
    let map = LabMap::new(INPUT);

    let part_two = map.part_two();
    let part_one = map.part_one();

    println!("=== Day 6 ===");

    println!();
    println!("Part One:");
    println!("{}", part_one);

    println!();
    println!("Part Two:");
    println!("{}", part_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_ONE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn example_1() {
        let map = LabMap::new(EXAMPLE_ONE);
        assert_eq!(map.part_one(), 41);
    }

    #[test]
    fn example_2() {
        let map = LabMap::new(EXAMPLE_ONE);
        assert_eq!(map.part_two(), 6);
    }

    #[test]
    fn part_one_final() {
        let map = LabMap::new(INPUT);
        assert_eq!(map.part_one(), 4964);
    }

    #[test]
    fn part_two_final() {
        let map = LabMap::new(INPUT);
        assert_eq!(map.part_two(), 1740);
    }
}
