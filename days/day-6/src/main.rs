use std::{cell::OnceCell, collections::HashSet};

const INPUT: &str = include_str!("../input");

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

    fn amount_visted(&self) -> u32 {
        self.visted.len() as u32
    }

    fn part_one(mut self) -> u32 {
        self.travel_all();
        self.amount_visted()
    }
}

fn main() {
    let map = LabMap::new(INPUT);

    println!("=== Day 6 ===");

    println!();
    println!("Part One:");
    println!("{}", map.part_one());
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
}
