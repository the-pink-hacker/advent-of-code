use common::*;

include_input!(INPUT);

trait IsBox {
    fn is_box(&self) -> bool;
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

    fn widen(value: Option<Self>) -> [Option<WideTile>; 2] {
        match value {
            Some(Self::Wall) => [Some(WideTile::Wall), Some(WideTile::Wall)],
            Some(Self::Box) => [Some(WideTile::BoxLeft), Some(WideTile::BoxRight)],
            None => [None, None],
        }
    }
}

impl IsBox for Tile {
    fn is_box(&self) -> bool {
        match self {
            Self::Box => true,
            Self::Wall => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum WideTile {
    Wall,
    BoxLeft,
    BoxRight,
}

impl IsBox for WideTile {
    fn is_box(&self) -> bool {
        match self {
            Self::Wall | Self::BoxRight => false,
            Self::BoxLeft => true,
        }
    }
}

trait Warehouse<T: IsBox> {
    fn get_tile(&self, position: (u8, u8)) -> Option<&T> {
        let (x, y) = position;
        self.get_tiles()
            .get(y as usize)
            .and_then(|tiles| tiles.get(x as usize).and_then(Option::as_ref))
    }

    fn push(&mut self, position: (u8, u8), direction: Direction) -> bool;

    fn step(&mut self, direction: Direction);

    fn box_gps_sum(&self) -> u32 {
        width_height_2d_iter(self.get_width(), self.get_height())
            .filter(|position| self.is_box(*position))
            .map(Self::gps_location)
            .sum()
    }

    fn get_tiles(&self) -> &Vec<Vec<Option<T>>>;

    fn get_tiles_mut(&mut self) -> &mut Vec<Vec<Option<T>>>;

    fn get_directions(&self) -> &Vec<Direction>;

    fn get_width(&self) -> u8;

    fn get_height(&self) -> u8;

    fn is_box(&self, position: (u8, u8)) -> bool {
        let (x, y) = position;
        self.get_tiles()[y as usize][x as usize]
            .as_ref()
            .map(IsBox::is_box)
            .unwrap_or_default()
    }

    fn push_tile(&mut self, from: (u8, u8), to: (u8, u8)) {
        let (from_x, from_y) = from;
        let (to_x, to_y) = to;
        let tile = self.get_tiles_mut()[from_y as usize][from_x as usize]
            .take()
            .unwrap();
        self.get_tiles_mut()[to_y as usize][to_x as usize] = Some(tile);
    }

    fn step_all(&mut self) -> u32 {
        self.get_directions()
            .clone()
            .into_iter()
            .for_each(|direction| self.step(direction));

        self.box_gps_sum()
    }

    fn gps_location(position: (u8, u8)) -> u32 {
        let (x, y) = position;
        x as u32 + y as u32 * 100
    }
}

#[derive(Debug, Clone)]
struct NormalWarehouse {
    tiles: Vec<Vec<Option<Tile>>>,
    width: u8,
    height: u8,
    robot: (u8, u8),
    directions: Vec<Direction>,
}

impl NormalWarehouse {
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

    fn widen(self) -> WideWarehouse {
        let (x, y) = self.robot;
        let robot = (x * 2 - 1, y);

        let tiles = self
            .tiles
            .into_iter()
            .map(|line| line.into_iter().flat_map(Tile::widen).collect())
            .collect();

        WideWarehouse {
            width: self.width * 2,
            height: self.height,
            directions: self.directions,
            tiles,
            robot,
        }
    }
}

impl Warehouse<Tile> for NormalWarehouse {
    fn push(&mut self, position: (u8, u8), direction: Direction) -> bool {
        let push_position = direction.apply(position);

        let can_push = match self.get_tile(push_position) {
            Some(Tile::Wall) => false,
            Some(Tile::Box) => self.push(push_position, direction),
            None => true,
        };

        if can_push {
            self.push_tile(position, push_position);
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

    fn get_tiles(&self) -> &Vec<Vec<Option<Tile>>> {
        &self.tiles
    }

    fn get_tiles_mut(&mut self) -> &mut Vec<Vec<Option<Tile>>> {
        &mut self.tiles
    }

    fn get_directions(&self) -> &Vec<Direction> {
        &self.directions
    }

    fn get_width(&self) -> u8 {
        self.width
    }

    fn get_height(&self) -> u8 {
        self.height
    }
}

struct WideWarehouse {
    tiles: Vec<Vec<Option<WideTile>>>,
    width: u8,
    height: u8,
    robot: (u8, u8),
    directions: Vec<Direction>,
}

impl WideWarehouse {
    fn can_push_vertical(&self, position: (u8, u8), direction: Direction) -> bool {
        false
    }

    fn push_vertical(&mut self, position: (u8, u8), direction: Direction) {
        let push_position = direction.apply(position);

        self.push_vertical(push_position, direction);
        self.push_tile(position, push_position);

        let other_direction = self.get_tile(push_position).map(|tile| match tile {
            WideTile::Wall => panic!("Hit wall: {:?}", push_position),
            WideTile::BoxLeft => Direction::Right,
            WideTile::BoxRight => Direction::Left,
        });

        if let Some(other_direction) = other_direction {
            let other_push_positon = other_direction.apply(push_position);
            self.push_vertical(other_push_positon, direction);
        }
    }
}

impl Warehouse<WideTile> for WideWarehouse {
    fn push(&mut self, position: (u8, u8), direction: Direction) -> bool {
        let push_position = direction.apply(position);
        let push_tile = self.get_tile(push_position);

        if direction.is_horizontal() {
            let can_push = match push_tile {
                Some(WideTile::Wall) => false,
                Some(WideTile::BoxLeft | WideTile::BoxRight) => self.push(push_position, direction),
                None => true,
            };

            if can_push {
                self.push_tile(position, push_position);
            }

            can_push
        } else {
            let can_push_forward = match push_tile {
                Some(WideTile::Wall) => false,
                Some(WideTile::BoxLeft | WideTile::BoxRight) => {
                    self.can_push_vertical(push_position, direction)
                }
                None => true,
            };

            if !can_push_forward {
                return false;
            }

            let current_tile = self.get_tile(position);

            let other_direction = match current_tile {
                Some(WideTile::BoxLeft) => Direction::Right,
                Some(WideTile::BoxRight) => Direction::Left,
                Some(WideTile::Wall) | None => panic!("Wall mising other pair: {:?}", position),
            };

            let other_position = other_direction.apply(position);
            let other_push_position = direction.apply(other_position);

            let can_push_other = match self.get_tile(other_push_position) {
                Some(WideTile::Wall) => false,
                Some(WideTile::BoxLeft | WideTile::BoxRight) => {
                    self.can_push_vertical(other_push_position, direction)
                }
                None => true,
            };

            if can_push_other {
                self.push_vertical(position, direction);
                self.push_vertical(other_position, direction);
                true
            } else {
                false
            }
        }
    }

    fn step(&mut self, direction: Direction) {
        let new_position = direction.apply(self.robot);

        match self.get_tile(new_position) {
            Some(WideTile::Wall) => return,
            Some(WideTile::BoxRight | WideTile::BoxLeft) => {
                if !self.push(new_position, direction) {
                    return;
                }
            }
            None => (),
        };

        self.robot = new_position;
    }

    fn gps_location(position: (u8, u8)) -> u32 {
        let (x, y) = position;
        (x - 1) as u32 + y as u32 * 100
    }

    fn get_tiles(&self) -> &Vec<Vec<Option<WideTile>>> {
        &self.tiles
    }

    fn get_tiles_mut(&mut self) -> &mut Vec<Vec<Option<WideTile>>> {
        &mut self.tiles
    }

    fn get_directions(&self) -> &Vec<Direction> {
        &self.directions
    }

    fn get_width(&self) -> u8 {
        self.width
    }

    fn get_height(&self) -> u8 {
        self.height
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

    fn is_horizontal(&self) -> bool {
        match self {
            Self::Up | Self::Down => false,
            Self::Left | Self::Right => true,
        }
    }
}

fn main() {
    let warehouse = NormalWarehouse::from_raw(INPUT);

    advent_solution(
        2024,
        15,
        warehouse.clone().step_all(),
        warehouse.widen().step_all(),
    );
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
        let mut warehouse = NormalWarehouse::from_raw(EXAMPLE_ONE);
        assert_eq!(warehouse.step_all(), 10092);
    }

    #[test]
    fn example_2() {
        let mut warehouse = NormalWarehouse::from_raw(EXAMPLE_TWO);
        assert_eq!(warehouse.step_all(), 2028);
    }

    #[test]
    fn example_3() {
        let mut warehouse = NormalWarehouse::from_raw(EXAMPLE_ONE).widen();
        assert_eq!(warehouse.step_all(), 9021);
    }

    #[test]
    fn part_one_final() {
        let mut warehouse = NormalWarehouse::from_raw(INPUT);
        assert_eq!(warehouse.step_all(), 1563092);
    }
}
