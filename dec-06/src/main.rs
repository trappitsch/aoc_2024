#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
enum Status {
    InsidePattern,
    OutsidePattern,
    InLoop,
}

struct WalkingPath {
    pattern: Vec<Vec<char>>,
    starting_position: (usize, usize),
    covered: Vec<Vec<usize>>,
    covered_directions: Vec<Vec<Vec<Direction>>>,
    current_position: (usize, usize), // (row, col)
    current_direction: Direction,
    status: Status,
    original_pattern: Vec<Vec<char>>,
}

impl WalkingPath {
    fn new(pattern: &str) -> WalkingPath {
        let pattern: Vec<Vec<char>> = pattern.lines().map(|l| l.chars().collect()).collect();
        let original_pattern = pattern.clone();
        let mut covered = vec![vec![0; pattern[0].len()]; pattern.len()];
        let covered_directions: Vec<Vec<Vec<Direction>>> = vec![vec![vec![]; pattern[0].len()]; pattern.len()];
        let row = pattern.iter().position(|r| r.contains(&'^')).unwrap();
        let col = pattern[row].iter().position(|c| c == &'^').unwrap();
        covered[row][col] = 1;
        WalkingPath {
            pattern,
            starting_position: (row, col),
            covered,
            covered_directions,
            current_position: (row, col),
            current_direction: Direction::Up,
            status: Status::InsidePattern,
            original_pattern,
        }
    }

    /// Move one step forward. Three things can happen:
    /// 1. Next step is not an obstacle (#), move forward and mark it as covered.
    /// 2. Next step is an obstacle, change direction to the right.
    /// 3. Next step is outside of pattern, update status to OutsidePattern.
    /// 4. Next step is on a field an in a direction that has been visited before, update status to
    ///    InLoop
    fn one_step(&mut self) {
        let (row, col) = self.current_position;
        let row = row as isize;
        let col = col as isize;
        let (next_row, next_col) = match self.current_direction {
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
        };
        if next_row < 0
            || next_col < 0
            || next_row > self.pattern.len() as isize - 1
            || next_col > self.pattern[0].len() as isize - 1
        {
            self.status = Status::OutsidePattern;
        } else if self.pattern[next_row as usize][next_col as usize] == '#' {
            self.current_direction = match self.current_direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
        } else {
            self.current_position = (next_row as usize, next_col as usize);
            self.covered[next_row as usize][next_col as usize] = 1;

            if self.covered_directions[next_row as usize][next_col as usize].iter().any(|d| d == &self.current_direction) {
                self.status = Status::InLoop;
            } else {
                self.covered_directions[next_row as usize][next_col as usize].push(self.current_direction.clone());
            }
        }
    }

    fn sum_covered(&self) -> usize {
        self.covered.iter().flatten().sum()
    }

    fn is_loop(&mut self, obst_row: usize, obst_col: usize) -> bool {
        if self.starting_position == (obst_row, obst_col) || self.pattern[obst_row][obst_col] == '#' {
            false
        } else {
            self.reset();
            let mut pattern = self.original_pattern.clone();
            pattern[obst_row][obst_col] = '#';
            self.pattern = pattern;
            loop{
                self.one_step();
                match self.status {
                    Status::InLoop => return true,
                    Status::OutsidePattern => return false,
                    _ => (),
                }
            }
        }
    }

    fn count_loops(&mut self) -> usize {
        let mut loops = 0;
        for nrow in 0..self.pattern.len() {
            for ncol in 0..self.pattern[0].len() {
                if self.is_loop(nrow, ncol) {
                    loops += 1;
                }
            }
        }
        loops
    }

    fn reset(&mut self) {
        self.covered = vec![vec![0; self.pattern[0].len()]; self.pattern.len()];
        self.covered_directions = vec![vec![vec![]; self.pattern[0].len()]; self.pattern.len()];
        self.current_position = self.starting_position;
        self.current_direction = Direction::Up;
        self.status = Status::InsidePattern;
    }

    fn walk(&mut self) {
        while self.status == Status::InsidePattern {
            self.one_step();
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut path = WalkingPath::new(&input);
    path.walk();
    println!("Result Part 1: {}", path.sum_covered());
    println!("Result Part 2: {}", path.count_loops());
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAT: &str = "....#.....
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
    fn test_starting_position() {
        let path = WalkingPath::new(PAT);
        assert_eq!(path.current_position, (6, 4));
        assert_eq!(path.sum_covered(), 1);
    }

    #[test]
    fn test_part_one() {
        let mut path = WalkingPath::new(PAT);
        path.walk();
        assert_eq!(path.sum_covered(), 41);
    }

    #[test]
    fn test_part_two() {
        let mut path = WalkingPath::new(PAT);
        assert_eq!(path.count_loops(), 6);
    }
    
}
