use std::collections::{HashMap, HashSet};

use common::*;
use itertools::Itertools;

include_input!(INPUT);

fn reduce_slope(slope: (isize, isize)) -> (isize, isize) {
    let (mut x, mut y) = slope;

    let min_absolute = x.abs().min(y.abs());

    for i in (2..=min_absolute).rev() {
        let x_multiple = x % i == 0;
        let y_multiple = y % i == 0;

        if x_multiple && y_multiple {
            x /= i;
            y /= i;
        }
    }

    (x, y)
}

struct City {
    antennas: HashMap<char, Vec<(usize, usize)>>,
    width: usize,
    height: usize,
}

impl City {
    fn parse(raw: &str) -> Self {
        let mut antennas = HashMap::<_, Vec<_>>::new();
        let mut height = 0;
        let mut width = 0;

        for (y, line) in raw.lines().enumerate() {
            height += 1;
            if y == 0 {
                width = line.len();
            }

            for (x, character) in line
                .chars()
                .enumerate()
                .filter(|(_, character)| character.is_ascii_alphanumeric())
            {
                antennas.entry(character).or_default().push((x, y));
            }
        }

        Self {
            antennas,
            width,
            height,
        }
    }

    fn inline_antennas(
        &self,
        frequency: &char,
        position: (usize, usize),
    ) -> Vec<Vec<(usize, usize)>> {
        let mut slopes = HashMap::<_, Vec<_>>::new();

        if let Some(antennas) = self.antennas.get(frequency) {
            let (x, y) = position;

            for (antenna_x, antenna_y) in antennas {
                let slope_x = *antenna_x as isize - x as isize;
                let slope_y = *antenna_y as isize - y as isize;

                let slope = reduce_slope((slope_x, slope_y));

                slopes
                    .entry(slope)
                    .or_default()
                    .push((*antenna_x, *antenna_y));
            }
        }

        slopes
            .into_values()
            .filter(|antennas| antennas.len() > 1)
            .collect()
    }

    fn antinodes_distance(&self) -> u32 {
        let mut total_antinodes = 0;

        width_height_2d_iter(self.width, self.height).for_each(|(x, y)| {
            'frequency: for frequency in self.antennas.keys() {
                let inline_antennas = self.inline_antennas(frequency, (x, y));

                for inline_antenna_group in inline_antennas {
                    for antennas in inline_antenna_group.into_iter().combinations(2) {
                        let (antenna_a_x, _) = antennas[0];
                        let (antenna_b_x, _) = antennas[1];

                        let distance_a_x = antenna_a_x.abs_diff(x);
                        let distance_b_x = antenna_b_x.abs_diff(x);

                        let min_distance = distance_a_x.min(distance_b_x);
                        let max_distance = distance_a_x.max(distance_b_x);

                        if max_distance % min_distance == 0 && max_distance / min_distance == 2 {
                            total_antinodes += 1;
                            break 'frequency;
                        }
                    }
                }
            }
        });

        total_antinodes
    }

    fn antinodes(&self) -> u32 {
        let mut antinode_set = self
            .antennas
            .values()
            .flatten()
            .cloned()
            .collect::<HashSet<_>>();

        width_height_2d_iter(self.width, self.height).for_each(|(x, y)| {
            for frequency in self.antennas.keys() {
                let inline_antennas = self.inline_antennas(frequency, (x, y));

                if !inline_antennas.is_empty() {
                    antinode_set.insert((x, y));
                    break;
                }
            }
        });

        antinode_set.len() as u32
    }
}

fn main() {
    let city = City::parse(INPUT);

    advent_solution(8, city.antinodes_distance(), city.antinodes());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_ONE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn reduce_slope() {
        assert_eq!(super::reduce_slope((10, -20)), (1, -2));
    }

    #[test]
    fn example_1_inline() {
        let city = City::parse(EXAMPLE_ONE);
        let inlines = city.inline_antennas(&'A', (7, 7));
        dbg!(&inlines);
        assert_eq!(inlines.len(), 1);
        assert_eq!(inlines[0].len(), 2);
        assert!(inlines[0].contains(&(8, 8)));
        assert!(inlines[0].contains(&(9, 9)));
    }

    #[test]
    fn example_1() {
        let city = City::parse(EXAMPLE_ONE);
        assert_eq!(city.antinodes_distance(), 14);
    }

    #[test]
    fn example_2() {
        let city = City::parse(EXAMPLE_ONE);
        assert_eq!(city.antinodes(), 34);
    }

    #[test]
    fn part_one_final() {
        let city = City::parse(INPUT);
        assert_eq!(city.antinodes_distance(), 376);
    }

    #[test]
    fn part_two_final() {
        let city = City::parse(INPUT);
        assert_eq!(city.antinodes(), 1352);
    }
}
