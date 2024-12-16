use std::collections::{HashMap, HashSet};

fn main() {
    let data = std::fs::read_to_string("input").unwrap();
    let mut grid = Grid::new(&data);

    grid.calc_antipodes().unwrap();
    println!("Result part 1: {}", grid.antipodes.len());

    grid.calc_antipodes_new().unwrap();
    println!("Results part 2: {}", grid.antipodes.len());
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, PartialOrd)]
struct Point {
    row: i64,
    col: i64,
}

impl Point {
    /// Calculate antipodes that are not negative or larger than max.
    fn antipodes(&self, p2: Point, max: Point) -> Vec<Point> {
        if *self == p2 {
            panic!("You did something wrong..., the two points are the same.");
        }

        let dcol = p2.col - self.col;
        let drow = p2.row - self.row;

        let ap1 = Point {
            col: self.col - dcol,
            row: self.row - drow,
        };
        let ap2 = Point {
            col: p2.col + dcol,
            row: p2.row + drow,
        };

        let mut result = Vec::new();
        if !ap1.has_negative() && !ap1.outside(max) {
            result.push(ap1);
        }
        if !ap2.has_negative() && !ap2.outside(max) {
            result.push(ap2);
        }
        result
    }

    fn antipodes_new(&self, p2: Point, max: Point) -> Vec<Point> {
        let dcol = p2.col - self.col;
        let drow = p2.row - self.row;

        // find largest common divisor

        let mut mcol = 1;
        let mut mrow = 1;

        for div in (1..=drow.abs()).rev() {
            if dcol % div == 0 && drow % div == 0 {
                mcol = dcol / div;
                mrow = drow / div;
                break;
            }
        }

        let mut result = Vec::new();

        let mut mpt = Point {
            col: self.col,
            row: self.row,
        };

        // negative loop
        loop {
            if mpt.is_in_grid(max) {
                result.push(mpt);
            } else {
                break;
            }
            mpt.col -= mcol;
            mpt.row -= mrow;
        }

        let mut mpt = Point {
            col: self.col,
            row: self.row,
        };
        // positive loop
        loop {
            mpt.col += mcol;
            mpt.row += mrow;

            if mpt.is_in_grid(max) {
                result.push(mpt);
            } else {
                break;
            }
        }


        result
    }

    fn is_in_grid(&self, max: Point) -> bool {
        if self.has_negative() || self.outside(max) {
            return false;
        }
        true
    }

    fn outside(&self, max: Point) -> bool {
        if self.row > max.row || self.col > max.col {
            return true;
        }
        false
    }

    fn has_negative(&self) -> bool {
        if self.row < 0 || self.col < 0 {
            return true;
        }
        false
    }
}

/// Grid that holds all the coordinate points for antennas and antipodes (once calculated).
/// The max point describes the positive maximum, the min point is always (0, 0).
struct Grid {
    antennas: HashMap<char, Vec<Point>>,
    antipodes: HashSet<Point>,
    max: Point,
}

impl Grid {
    fn new(data: &str) -> Grid {
        let mut antennas = HashMap::new();
        let mut rows = 0;
        let mut cols = 0;
        for (lt, line) in data.lines().enumerate() {
            if lt > rows {
                rows = lt;
            }
            for (ct, ch) in line.chars().enumerate() {
                if ct > cols {
                    cols = ct;
                }
                if ch != '.' {
                    let pt = Point {
                        col: lt as i64,
                        row: ct as i64,
                    };
                    antennas
                        .entry(ch)
                        .and_modify(|entry: &mut Vec<Point>| entry.push(pt))
                        .or_insert(vec![pt]);
                }
            }
        }

        let antipodes = HashSet::new();
        let max = Point {
            row: rows as i64,
            col: cols as i64,
        };

        Grid {
            antennas,
            antipodes,
            max,
        }
    }

    fn calc_antipodes(&mut self) -> Result<(), String> {
        for items in self.antennas.values() {
            for (it, p1) in items.iter().enumerate() {
                for p2 in items[it + 1..].iter() {
                    let aps = p1.antipodes(*p2, self.max);
                    for ap in aps {
                        self.antipodes.insert(ap);
                    }
                }
            }
        }
        Ok(())
    }

    fn calc_antipodes_new(&mut self) -> Result<(), String> {
        for items in self.antennas.values() {
            for (it, p1) in items.iter().enumerate() {
                for p2 in items[it + 1..].iter() {
                    let aps = p1.antipodes_new(*p2, self.max);
                    for ap in aps {
                        self.antipodes.insert(ap);
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "............
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

    const DATA_T: &str = "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";

    #[test]
    fn test_some_points() {
        let a = Point { row: 1, col: 1 };
        let b = Point { row: 2, col: 2 };
        assert!(a < b);
        let c = Point { row: 1, col: 3 };
        assert!(a <= c);
        assert!(b.outside(a));
        assert!(b.outside(c));
        assert!(c.outside(b));
    }

    #[test]
    fn test_create_grid() {
        let grid = Grid::new(DATA);
        let vec_0 = grid.antennas.get(&'0').unwrap();
        let vec_a = grid.antennas.get(&'A').unwrap();
        let p0 = Point { col: 1, row: 8 };
        let pa = Point { col: 9, row: 9 };
        // Make sure p0 is in vec_0
        assert!(vec_0.iter().any(|x| *x == p0));
        assert!(vec_a.iter().any(|x| *x == pa));
    }

    #[test]
    fn test_get_antipode() {
        let p1 = Point { col: 10, row: 12 };
        let p2 = Point { col: 16, row: 20 };
        let ap1_exp = Point { col: 4, row: 4 };
        let ap2_exp = Point { col: 22, row: 28 };
        let max = Point { col: 100, row: 100 };

        let res1 = p1.antipodes(p2, max);
        let res2 = p2.antipodes(p1, max);

        assert!(res1.iter().any(|x| ap1_exp == *x));
        assert!(res1.iter().any(|x| ap2_exp == *x));
        assert!(res2.iter().any(|x| ap1_exp == *x));
        assert!(res2.iter().any(|x| ap2_exp == *x));
    }

    #[test]
    fn test_part1() {
        let mut grid = Grid::new(DATA);
        grid.calc_antipodes().unwrap();
        assert_eq!(grid.antipodes.len(), 14);
    }

    #[test]
    fn test_data_t() {
        let mut grid = Grid::new(DATA_T);
        grid.calc_antipodes_new().unwrap();
        assert_eq!(grid.antipodes.len(), 9);
    }

    #[test]
    fn test_part2() {
        let mut grid = Grid::new(DATA);
        grid.calc_antipodes_new().unwrap();
        assert_eq!(grid.antipodes.len(), 34);
    }
}
