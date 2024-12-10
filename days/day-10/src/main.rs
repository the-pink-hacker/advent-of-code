use std::collections::HashSet;

use common::*;

include_input!(INPUT);

struct HikingMap {
    grid: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl HikingMap {
    fn new(raw: &str) -> Self {
        let grid = raw
            .lines()
            .map(|line| {
                line.chars()
                    .map(|character| {
                        character
                            .to_digit(10)
                            .and_then(|digit| digit.try_into().ok())
                    })
                    .map(Option::unwrap)
                    .collect()
            })
            .collect::<Vec<Vec<_>>>();

        Self {
            width: grid.len(),
            height: grid[0].len(),
            grid,
        }
    }

    fn scan_visted(&self, x: usize, y: usize, found: &mut HashSet<(usize, usize)>) {
        let current = self.grid[y][x];

        if current == 9 {
            found.insert((x, y));
        } else {
            if x > 0 && self.grid[y][x - 1].wrapping_sub(current) == 1 {
                self.scan_visted(x - 1, y, found);
            }

            if x < self.width - 1 && self.grid[y][x + 1].wrapping_sub(current) == 1 {
                self.scan_visted(x + 1, y, found);
            }

            if y > 0 && self.grid[y - 1][x].wrapping_sub(current) == 1 {
                self.scan_visted(x, y - 1, found);
            }

            if y < self.height - 1 && self.grid[y + 1][x].wrapping_sub(current) == 1 {
                self.scan_visted(x, y + 1, found);
            }
        }
    }

    fn scan_path(&self, x: usize, y: usize, found: &mut usize) {
        let current = self.grid[y][x];

        if current == 9 {
            *found += 1;
        } else {
            if x > 0 && self.grid[y][x - 1].wrapping_sub(current) == 1 {
                self.scan_path(x - 1, y, found);
            }

            if x < self.width - 1 && self.grid[y][x + 1].wrapping_sub(current) == 1 {
                self.scan_path(x + 1, y, found);
            }

            if y > 0 && self.grid[y - 1][x].wrapping_sub(current) == 1 {
                self.scan_path(x, y - 1, found);
            }

            if y < self.height - 1 && self.grid[y + 1][x].wrapping_sub(current) == 1 {
                self.scan_path(x, y + 1, found);
            }
        }
    }

    fn part_one(&self) -> usize {
        width_height_2d_iter(self.width, self.height)
            .filter(|(x, y)| self.grid[*y][*x] == 0)
            .map(|(x, y)| {
                let mut visted = HashSet::new();

                self.scan_visted(x, y, &mut visted);

                visted.len()
            })
            .sum()
    }

    fn part_two(&self) -> usize {
        width_height_2d_iter(self.width, self.height)
            .filter(|(x, y)| self.grid[*y][*x] == 0)
            .map(|(x, y)| {
                let mut found = 0;
                self.scan_path(x, y, &mut found);
                found
            })
            .sum()
    }
}

fn main() {
    let map = HikingMap::new(INPUT);

    advent_solution(10, map.part_one(), map.part_two());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_ONE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn example_1() {
        let map = HikingMap::new(EXAMPLE_ONE);
        assert_eq!(map.part_one(), 36);
    }

    #[test]
    fn example_2() {
        let map = HikingMap::new(EXAMPLE_ONE);
        assert_eq!(map.part_two(), 81);
    }

    #[test]
    fn part_one_final() {
        let map = HikingMap::new(INPUT);
        assert_eq!(map.part_one(), 822);
    }

    #[test]
    fn path_two_final() {
        let map = HikingMap::new(INPUT);
        assert_eq!(map.part_two(), 1801);
    }
}
