use common::*;

include_input!(INPUT);

fn gps_location(position: (u8, u8)) -> u32 {
    let (x, y) = position;
    x as u32 + y as u32 * 100
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Wall,
    Box,
}

impl Tile {
    fn from_char(value: char) -> Option<Self> {
        match value {
            '#' => Some(Self::Wall),
            'O' => Some(Self::Box),
            _ => None,
        }
    }

    fn is_box(&self) -> bool {
        match self {
            Self::Box => true,
            Self::Wall => false,
        }
    }
}

struct Warehouse {
    tiles: Vec<Vec<Option<Tile>>>,
    width: u8,
    height: u8,
    robot: (u8, u8),
    directions: Vec<Direction>,
}

impl Warehouse {
    fn from_raw(raw: &str) -> Self {
        let (tiles, movements) = raw.split_once("\n\n").unwrap();

        let mut robot = (0, 0);

        let tiles = tiles
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, value)| {
                        if value == '@' {
                            robot = (x as u8, y as u8)
                        }
                        value
                    })
                    .map(Tile::from_char)
                    .collect()
            })
            .collect::<Vec<Vec<_>>>();
        let height = tiles.len() as u8;
        let width = tiles[0].len() as u8;

        let directions = movements.chars().filter_map(Direction::from_char).collect();

        Self {
            tiles,
            width,
            height,
            robot,
            directions,
        }
    }

    fn get_tile(&self, position: (u8, u8)) -> Option<&Tile> {
        let (x, y) = position;
        self.tiles
            .get(y as usize)
            .and_then(|tiles| tiles.get(x as usize).and_then(Option::as_ref))
    }

    fn push(&mut self, position: (u8, u8), direction: Direction) -> bool {
        let push_position = direction.apply(position);

        let can_push = match self.get_tile(push_position) {
            Some(Tile::Wall) => false,
            Some(Tile::Box) => self.push(push_position, direction),
            None => true,
        };

        if can_push {
            let (x, y) = position;
            let (push_x, push_y) = push_position;

            let tile = self.tiles[y as usize][x as usize].take().unwrap();
            self.tiles[push_y as usize][push_x as usize] = Some(tile);
        }

        can_push
    }

    fn step(&mut self, direction: Direction) {
        let new_position = direction.apply(self.robot);

        match self.get_tile(new_position) {
            Some(Tile::Wall) => return,
            Some(Tile::Box) => {
                if !self.push(new_position, direction) {
                    return;
                }
            }
            None => (),
        };

        self.robot = new_position;
    }

    fn is_box(&self, position: (u8, u8)) -> bool {
        let (x, y) = position;
        self.tiles[y as usize][x as usize]
            .as_ref()
            .map(Tile::is_box)
            .unwrap_or_default()
    }

    fn box_gps_sum(&self) -> u32 {
        width_height_2d_iter(self.width, self.height)
            .filter(|position| self.is_box(*position))
            .map(gps_location)
            .sum()
    }

    fn part_one(&mut self) -> u32 {
        self.directions
            .clone()
            .into_iter()
            .for_each(|direction| self.step(direction));

        self.box_gps_sum()
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(value: char) -> Option<Self> {
        match value {
            '^' => Some(Self::Up),
            'v' => Some(Self::Down),
            '<' => Some(Self::Left),
            '>' => Some(Self::Right),
            _ => None,
        }
    }

    fn apply(&self, position: (u8, u8)) -> (u8, u8) {
        let (x, y) = position;
        let (direction_x, direction_y) = match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        };

        let new_x = x.wrapping_add_signed(direction_x);
        let new_y = y.wrapping_add_signed(direction_y);

        (new_x, new_y)
    }
}

fn main() {
    let mut warehouse = Warehouse::from_raw(INPUT);

    advent_solution(2024, 15, warehouse.part_one(), "");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_ONE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    const EXAMPLE_TWO: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    #[test]
    fn example_1() {
        let mut warehouse = Warehouse::from_raw(EXAMPLE_ONE);
        assert_eq!(warehouse.part_one(), 10092);
    }

    #[test]
    fn example_2() {
        let mut warehouse = Warehouse::from_raw(EXAMPLE_TWO);
        assert_eq!(warehouse.part_one(), 2028);
    }

    #[test]
    fn part_one_final() {
        let mut warehouse = Warehouse::from_raw(INPUT);
        assert_eq!(warehouse.part_one(), 1563092);
    }
}
