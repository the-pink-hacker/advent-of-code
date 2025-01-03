use std::collections::HashSet;

use common::*;

include_input!(INPUT);

const UP: (i8, i8) = (0, -1);
const DOWN: (i8, i8) = (0, 1);
const LEFT: (i8, i8) = (-1, 0);
const RIGHT: (i8, i8) = (1, 0);

struct Garden {
    plots: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

fn pop_set(set: &mut HashSet<(usize, usize)>) -> Option<(usize, usize)> {
    let element = set.iter().next().cloned()?;
    set.remove(&element);
    Some(element)
}

impl Garden {
    fn new(raw: &str) -> Self {
        let plots = raw
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<_>>>();
        let width = plots[0].len();
        let height = plots.len();

        Self {
            plots,
            width,
            height,
        }
    }

    fn get_offset(&self, position: (usize, usize), offset: (i8, i8)) -> Option<char> {
        let (x, y) = position;
        let (offset_x, offset_y) = offset;

        let new_x = x.checked_add_signed(offset_x as isize)?;
        let new_y = y.checked_add_signed(offset_y as isize)?;

        if new_x < self.width && new_y < self.height {
            Some(self.plots[new_y][new_x])
        } else {
            None
        }
    }

    fn is_plot_same(&self, position: (usize, usize), offset: (i8, i8)) -> bool {
        let (x, y) = position;

        self.get_offset(position, offset)
            .map(|plot| self.plots[y][x] == plot)
            .unwrap_or_default()
    }

    fn scan_region(
        &self,
        position: (usize, usize),
        visted: &mut HashSet<(usize, usize)>,
    ) -> Region {
        let mut area = 1;
        let mut perimeter = 0;
        visted.insert(position);
        let mut to_check = [position].into_iter().collect::<HashSet<_>>();

        while let Some(check_position) = pop_set(&mut to_check) {
            for direction in [UP, DOWN, LEFT, RIGHT] {
                if self.is_plot_same(check_position, direction) {
                    let (x, y) = check_position;
                    let (direction_x, direction_y) = direction;
                    let new_x = x.wrapping_add_signed(direction_x as isize);
                    let new_y = y.wrapping_add_signed(direction_y as isize);

                    if !visted.contains(&(new_x, new_y)) {
                        area += 1;
                        visted.insert((new_x, new_y));
                        to_check.insert((new_x, new_y));
                    }
                } else {
                    perimeter += 1;
                }
            }
        }

        Region { area, perimeter }
    }

    fn scan_side(
        &self,
        position: (usize, usize),
        direction: (i8, i8),
    ) -> ((usize, usize), (usize, usize)) {
        let (mut x, mut y) = position;
        let current_plot = self.plots[y][x];
        let (direction_x, direction_y) = direction;
        let scan_direction = (direction_y, direction_x);

        let mut positive_end = position;

        while let Some(search_plot) = self.get_offset((x, y), scan_direction) {
            x = x.wrapping_add_signed(scan_direction.0 as isize);
            y = y.wrapping_add_signed(scan_direction.1 as isize);

            if search_plot != current_plot || self.is_plot_same((x, y), direction) {
                positive_end = (x, y);
            }
        }

        let scan_direction = (-direction_y, -direction_x);

        let mut negative_end = position;

        while let Some(search_plot) = self.get_offset((x, y), scan_direction) {
            x = x.wrapping_add_signed(scan_direction.0 as isize);
            y = y.wrapping_add_signed(scan_direction.1 as isize);

            if search_plot != current_plot || self.is_plot_same((x, y), direction) {
                negative_end = (x, y);
            }
        }

        (
            (
                positive_end.0.min(negative_end.0),
                positive_end.1.min(negative_end.1),
            ),
            (
                positive_end.0.max(negative_end.0),
                positive_end.1.max(negative_end.1),
            ),
        )
    }

    fn scan_region_sides(
        &self,
        position: (usize, usize),
        visted: &mut HashSet<(usize, usize)>,
    ) -> Region {
        let mut area = 1;
        let mut sides = HashSet::<((usize, usize), (usize, usize))>::new();
        visted.insert(position);
        let mut to_check = [position].into_iter().collect::<HashSet<_>>();

        while let Some(check_position) = pop_set(&mut to_check) {
            for direction in [UP, DOWN, LEFT, RIGHT] {
                if self.is_plot_same(check_position, direction) {
                    let (x, y) = check_position;
                    let (direction_x, direction_y) = direction;
                    let new_x = x.wrapping_add_signed(direction_x as isize);
                    let new_y = y.wrapping_add_signed(direction_y as isize);

                    if !visted.contains(&(new_x, new_y)) {
                        area += 1;
                        visted.insert((new_x, new_y));
                        to_check.insert((new_x, new_y));
                    }
                } else {
                    let (side_lower, side_upper) = self.scan_side(check_position, direction);
                    if side_lower != side_upper {
                        sides.insert((side_lower, side_upper));
                    }
                }
            }
        }

        dbg!(self.plots[position.1][position.0], &sides);

        Region {
            area,
            perimeter: sides.len() as u32,
        }
    }

    fn regions(
        &self,
        scan: impl Fn(&Self, (usize, usize), &mut HashSet<(usize, usize)>) -> Region,
    ) -> u32 {
        let area = self.width * self.height;
        let mut visted = HashSet::<(usize, usize)>::with_capacity(area);

        width_height_2d_iter(self.width, self.height)
            .filter_map(|position| {
                if !visted.contains(&position) {
                    Some(scan(self, position, &mut visted))
                } else {
                    None
                }
            })
            .map(|region| region.cost())
            .sum()
    }

    fn part_one(&self) -> u32 {
        self.regions(Self::scan_region)
    }

    fn part_two(&self) -> u32 {
        self.regions(Self::scan_region_sides)
    }
}

#[derive(Debug)]
struct Region {
    perimeter: u32,
    area: u32,
}

impl Region {
    fn cost(&self) -> u32 {
        self.perimeter * self.area
    }
}

fn main() {
    let garden = Garden::new(INPUT);

    advent_solution(2024, 12, garden.part_one(), garden.part_two());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_ONE: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const EXAMPLE_TWO: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    #[test]
    fn example_1() {
        let garden = Garden::new(EXAMPLE_ONE);
        assert_eq!(garden.part_one(), 1930);
    }

    #[test]
    fn example_2() {
        let garden = Garden::new(EXAMPLE_TWO);
        assert_eq!(garden.part_two(), 236);
    }

    #[test]
    fn part_one_final() {
        let garden = Garden::new(INPUT);
        assert_eq!(garden.part_one(), 1550156);
    }
}
