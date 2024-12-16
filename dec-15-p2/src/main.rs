const DATA_SMALL: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

fn main() {
    let mut map = Map::from_string(DATA_SMALL);
    println!("{}", map);
    //for dir in map.moves.clone() {
    //    map.next_move(dir);
    //    println!("{}", map);
    //}

    //    let data = std::fs::read_to_string("input").unwrap();
    //    let mut map = Map::from_string(&data);
    //    map.move_all();
    //
    //    println!("Part 1: {}", map.sum_gps());
}

#[derive(Debug, Clone, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn from_char(c: char) -> Dir {
        match c {
            '^' => Dir::Up,
            'v' => Dir::Down,
            '<' => Dir::Left,
            '>' => Dir::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn new(row: usize, col: usize) -> Coord {
        Coord { row, col }
    }

    // Returns a new coord with the moved coordinates.
    fn predict_move(&self, dir: &Dir) -> Coord {
        match *dir {
            Dir::Up => Coord {
                row: self.row - 1,
                col: self.col,
            },
            Dir::Down => Coord {
                row: self.row + 1,
                col: self.col,
            },
            Dir::Left => Coord {
                row: self.row,
                col: self.col - 1,
            },
            Dir::Right => Coord {
                row: self.row,
                col: self.col + 1,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Field {
    Empty,
    Wall,
    BoxL,
    BoxR,
    Robot,
}

impl Field {
    fn from_char(c: char) -> Field {
        match c {
            '#' => Field::Wall,
            '.' => Field::Empty,
            '[' => Field::BoxL,
            ']' => Field::BoxR,
            '@' => Field::Robot,
            _ => panic!("Invalid field"),
        }
    }
}

struct Map {
    fields: Vec<Vec<Field>>,
    moves: Vec<Dir>,
    robot: Coord,
    total_moves: usize,
}

// implement display for map
impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.fields.iter() {
            for cell in row.iter() {
                match cell {
                    Field::Empty => write!(f, ".")?,
                    Field::Wall => write!(f, "#")?,
                    Field::BoxL => write!(f, "[")?,
                    Field::BoxR => write!(f, "]")?,
                    Field::Robot => write!(f, "@")?,
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "Total moves: {}", self.total_moves)
    }
}

impl Map {
    fn from_string(data: &str) -> Map {
        let mut fields = Vec::new();
        let mut moves = Vec::new();
        let mut robot = Coord::new(0, 0);

        let mut parsing_map = true;
        for (row, line) in data.lines().enumerate() {
            if parsing_map {
                if line.is_empty() {
                    parsing_map = false;
                    continue;
                }

                let mut rowvec = Vec::new();
                for (col, c) in line.chars().enumerate() {
                    let coord_left = Coord::new(row, 2 * col);
                    let mut field = Vec::new();
                    if c == 'O' {
                        field.push(Field::BoxL);
                        field.push(Field::BoxR);
                    } else if c == '@' {
                        field.push(Field::Robot);
                        field.push(Field::Empty);
                    } else {
                        field.push(Field::from_char(c));
                        field.push(Field::from_char(c));
                    }
                    if field[0] == Field::Robot{
                        robot = coord_left;
                    }
                    field.iter().for_each(|f| rowvec.push(f.clone()));
                }
                fields.push(rowvec);
            } else {
                for c in line.chars() {
                    moves.push(Dir::from_char(c));
                }
            }
        }

        assert!(robot != Coord::new(0, 0));

        Map {
            fields,
            moves,
            robot,
            total_moves: 0,
        }
    }

    /// Calculate sum of GPS
    fn sum_gps(&self) -> usize {
        let mut sum = 0;
        for (row, line) in self.fields.iter().enumerate() {
            for (col, field) in line.iter().enumerate() {
                if *field == Field::BoxL {
                    sum += row * 100 + col
                }
            }
        }
        sum
    }

    /// All moves
    fn move_all(&mut self) {
        for dir in self.moves.clone() {
            self.next_move(dir);
        }
    }

    /// Move the robot and all boxes (if there) for the next step, unless there's a wall.
    /// Update map.
    fn next_move(&mut self, dir: Dir) {
        let mut move_stack_coords = Vec::new();
        let mut do_move = false; // We assume there's a wall somewhere
        let mut coord = self.robot.clone();

        self.total_moves += 1;

        loop {
            let new_coord = coord.predict_move(&dir);
            let new_field = self.get_field(&new_coord);
            //println!("{:?} -> {:?}, nf: {:?}", coord, new_coord, new_field);
            match new_field {
                Field::Empty => {
                    move_stack_coords.push(coord.clone());
                    do_move = true;
                    break;
                }
                Field::Wall => {
                    break;
                }
                Field::Robot => {
                    panic!("The robot found another robot...");
                }
                _ => {
                    move_stack_coords.push(coord.clone());
                    coord = new_coord;
                }
            }
        }

        // Should we execute the move stack? If so, do it reversed
        if do_move {
            move_stack_coords.iter().rev().for_each(|c| {
                let field = self.get_field(c).clone();
                self.set_field(&c.predict_move(&dir), field);
            });
            self.set_field(&self.robot.clone(), Field::Empty);
            self.robot = self.robot.predict_move(&dir);
        }
    }

    fn set_field(&mut self, coord: &Coord, field: Field) {
        self.fields[coord.row][coord.col] = field;
    }

    fn get_field(&self, coord: &Coord) -> &Field {
        &self.fields[coord.row][coord.col]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_EX_LG: &str = "##########
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

    #[test]
    fn test_dir_from_char() {
        assert_eq!(Dir::from_char('^'), Dir::Up);
        assert_eq!(Dir::from_char('v'), Dir::Down);
        assert_eq!(Dir::from_char('<'), Dir::Left);
        assert_eq!(Dir::from_char('>'), Dir::Right);
    }

    #[test]
    #[should_panic]
    fn test_dir_from_char_invalid() {
        Dir::from_char('x');
    }

    #[test]
    fn test_coord_new() {
        assert_eq!(Coord::new(1, 2), Coord { row: 1, col: 2 });
    }

    #[test]
    fn test_p1_small() {
        let mut map = Map::from_string(DATA_SMALL);
        assert!(!map.fields.is_empty());
        assert!(!map.moves.is_empty());

        map.move_all();
        assert_eq!(map.sum_gps(), 2028);
    }

    #[test]
    fn test_p1() {
        let mut map = Map::from_string(DATA_EX_LG);
        map.move_all();
        assert_eq!(map.sum_gps(), 10092);
    }
}
