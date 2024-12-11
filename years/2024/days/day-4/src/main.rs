use common::*;

include_input!(INPUT);

#[derive(Debug)]
struct WordTable {
    table: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl WordTable {
    fn new(raw: &str) -> Self {
        let table = raw
            .lines()
            .map(|x| x.chars().collect())
            .collect::<Vec<Vec<_>>>();
        let height = table.len();
        let width = table.first().unwrap().len();

        Self {
            table,
            height,
            width,
        }
    }

    fn scan_xmas(&self) -> u32 {
        let search_value = "MAS";
        let search_length = search_value.len();

        width_height_2d_iter(self.width, self.height)
            .filter(|(x, y)| self.get_char(*x, *y) == 'X')
            .map(|(x, y)| {
                let mut found = 0;
                let allowed_up = y >= search_length;
                let allowed_left = x >= search_length;
                let allowed_down = y < self.height - search_length;
                let allowed_right = x < self.width - search_length;

                if allowed_down && self.scan(x, y, search_value, |_, y| *y += 1) {
                    found += 1;
                }

                if allowed_up && self.scan(x, y, search_value, |_, y| *y -= 1) {
                    found += 1;
                }

                if allowed_left && self.scan(x, y, search_value, |x, _| *x -= 1) {
                    found += 1;
                }

                if allowed_right && self.scan(x, y, search_value, |x, _| *x += 1) {
                    found += 1;
                }

                if allowed_right
                    && allowed_down
                    && self.scan(x, y, search_value, |x, y| {
                        *x += 1;
                        *y += 1
                    })
                {
                    found += 1;
                }

                if allowed_left
                    && allowed_up
                    && self.scan(x, y, search_value, |x, y| {
                        *x -= 1;
                        *y -= 1
                    })
                {
                    found += 1;
                }

                if allowed_left
                    && allowed_down
                    && self.scan(x, y, search_value, |x, y| {
                        *x -= 1;
                        *y += 1
                    })
                {
                    found += 1;
                }

                if allowed_right
                    && allowed_up
                    && self.scan(x, y, search_value, |x, y| {
                        *x += 1;
                        *y -= 1
                    })
                {
                    found += 1;
                }

                found
            })
            .sum()
    }

    fn scan_x_mas(&self) -> u32 {
        width_height_start_2d_iter(1, 1, self.width - 1, self.height - 1)
            .filter(|(x, y)| self.get_char(*x, *y) == 'A')
            .filter(|(x, y)| {
                let top_left = self.get_char(x - 1, y - 1);
                let top_right = self.get_char(x + 1, y - 1);
                let bottom_left = self.get_char(x - 1, y + 1);
                let bottom_right = self.get_char(x + 1, y + 1);

                let top_left_mas = top_left == 'M' && bottom_right == 'S';
                let bottom_right_mas = top_left == 'S' && bottom_right == 'M';
                let top_right_mas = top_right == 'M' && bottom_left == 'S';
                let bottom_left_mas = top_right == 'S' && bottom_left == 'M';

                (top_left_mas || bottom_right_mas) && (top_right_mas || bottom_left_mas)
            })
            .count() as u32
    }

    fn get_char(&self, x: usize, y: usize) -> char {
        *self.table.get(y).unwrap().get(x).unwrap()
    }

    fn scan(
        &self,
        mut x: usize,
        mut y: usize,
        value: &str,
        operation: impl Fn(&mut usize, &mut usize),
    ) -> bool {
        for value_char in value.chars() {
            operation(&mut x, &mut y);
            if self.get_char(x, y) != value_char {
                return false;
            }
        }

        true
    }
}

fn main() {
    let table = WordTable::new(INPUT);

    advent_solution(2024, 4, table.scan_xmas(), table.scan_x_mas());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_ONE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    const EXAMPLE_TWO: &str = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";

    #[test]
    fn example_1() {
        let table = WordTable::new(EXAMPLE_ONE);
        assert_eq!(table.scan_xmas(), 18);
    }

    #[test]
    fn example_2() {
        let table = WordTable::new(EXAMPLE_TWO);
        assert_eq!(table.scan_x_mas(), 9);
    }

    #[test]
    fn part_one_final() {
        let table = WordTable::new(INPUT);
        assert_eq!(table.scan_xmas(), 2378);
    }

    #[test]
    fn part_two_final() {
        let table = WordTable::new(INPUT);
        assert_eq!(table.scan_x_mas(), 1796);
    }
}
