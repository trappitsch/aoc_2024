use std::collections::HashSet;

fn main() {
    let data = std::fs::read_to_string("input").unwrap();
    let mut map = Map::new(&data);
    map.find_all_connected();
    println!("Part 1: {}", map.price_p1());
    println!("Part 2: {}", map.calc_price_p2());
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Coordinate {
    row: i64,
    col: i64,
}

struct GardenPlot {
    coordinates: HashSet<Coordinate>,
    sum_perimeter: u64,
    sides_p2: u64,
}

struct Map {
    data: Vec<Vec<char>>,
    plots: Vec<GardenPlot>,
}

impl Map {
    fn new(data: &str) -> Self {
        let data: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
        let plots = Vec::new();

        Self { data, plots }
    }

    /// Find all connected regions and add them to the plots vector.
    fn find_all_connected(&mut self) {
        while let Some(start_coord) = self.find_next_starting_coordinate() {
            let connected = self.find_connected(start_coord);
            let mut perimeter = 0;
            for crd in connected.iter() {
                perimeter += self.perimeter(*crd);
            }

            let plot = GardenPlot {
                coordinates: connected,
                sum_perimeter: perimeter,
                sides_p2: 0,
            };

            self.plots.push(plot);
        }
    }

    /// Loop through all existing coordinates and return one that is not yet in list
    fn find_next_starting_coordinate(&mut self) -> Option<Coordinate> {
        let mut visited: HashSet<Coordinate> = HashSet::new();
        for plt in self.plots.iter() {
            visited.extend(&plt.coordinates);
        }
        for row in 0..self.data.len() {
            for col in 0..self.data[row].len() {
                let coord_to_test = Coordinate {
                    row: row as i64,
                    col: col as i64,
                };
                if !visited.contains(&coord_to_test) {
                    return Some(coord_to_test);
                }
            }
        }
        None
    }
    ///
    /// Find regions that are connected from the given regions.
    fn find_connected(&self, start: Coordinate) -> HashSet<Coordinate> {
        let mut connected: HashSet<Coordinate> = HashSet::new();
        connected.insert(start);

        let mut len_connected = connected.len();

        loop {
            for crd in connected.clone().iter() {
                connected.extend(&self.find_adjacent(*crd));
            }
            if connected.len() == len_connected {
                break;
            }
            len_connected = connected.len();
        }

        connected
    }

    /// At a given tile, search adjacent tiles for the same character.
    fn find_adjacent(&self, coord: Coordinate) -> HashSet<Coordinate> {
        let mut adjacents: HashSet<Coordinate> = HashSet::new();
        let ch = self.get_char(coord);
        let (row, col) = (coord.row as usize, coord.col as usize);

        if row > 0 && self.data[row as usize - 1][col] == ch {
            adjacents.insert(Coordinate {
                row: row as i64 - 1,
                col: col as i64,
            });
        }
        if row < self.data.len() - 1 && self.data[row + 1][col] == ch {
            adjacents.insert(Coordinate {
                row: row as i64 + 1,
                col: col as i64,
            });
        }
        if col > 0 && self.data[row][col - 1] == ch {
            adjacents.insert(Coordinate {
                row: row as i64,
                col: col as i64 - 1,
            });
        }
        if col < self.data[row].len() - 1 && self.data[row][col + 1] == ch {
            adjacents.insert(Coordinate {
                row: row as i64,
                col: col as i64 + 1,
            });
        }
        adjacents
    }

    /// Get a character at a given coordinate.
    fn get_char(&self, coord: Coordinate) -> char {
        self.data[coord.row as usize][coord.col as usize]
    }

    /// Get the perimeter around a given point, 0 if the same, otherwise 1.
    fn perimeter(&self, coord: Coordinate) -> u64 {
        let (row, col) = (coord.row as usize, coord.col as usize);
        let mut peri = 0;
        let ch = self.data[row][col];
        // left
        if col == 0 || self.data[row][col - 1] != ch {
            peri += 1;
        }
        // right
        if col == self.data[row].len() - 1 || self.data[row][col + 1] != ch {
            peri += 1;
        }
        // above
        if row == 0 || self.data[row - 1][col] != ch {
            peri += 1;
        }
        // below
        if row == self.data.len() - 1 || self.data[row + 1][col] != ch {
            peri += 1;
        }
        peri
    }

    /// Calculate the price of all plots for part 1
    fn price_p1(&self) -> u64 {
        self.plots
            .iter()
            .map(|plt| plt.coordinates.len() as u64 * plt.sum_perimeter)
            .sum()
    }

    /// Calculate the number of connected sides from top to bottom.
    fn connected_sides_p2(&mut self) {
        for plt in self.plots.iter_mut() {
            plt.sides_p2 = 0;

            plt.sides_p2 = number_connected_sides_top_bottom(&plt.coordinates, false);

            plt.sides_p2 += number_connected_sides_top_bottom(&plt.coordinates, true);

            // rotate by flipping rows and columns
            let check: HashSet<Coordinate> = plt
                .coordinates
                .iter()
                .map(|crd| Coordinate {
                    row: crd.col,
                    col: crd.row,
                })
                .collect();
            plt.sides_p2 += number_connected_sides_top_bottom(&check, false);

            plt.sides_p2 += number_connected_sides_top_bottom(&check, true);
        }
    }

    fn calc_price_p2(&mut self) -> u64 {
        self.connected_sides_p2();
        self.plots
            .iter()
            .map(|plt| plt.sides_p2 * plt.coordinates.len() as u64)
            .sum()
    }
}

fn number_connected_sides_top_bottom(coordinates: &HashSet<Coordinate>, rev: bool) -> u64 {
    let mut min_row = 1_000_000_000;
    let mut max_row = -1_000_000_000;
    let mut min_col = 1_000_000_000;
    let mut max_col = -1_000_000_000;

    for crd in coordinates.iter() {
        if crd.row < min_row {
            min_row = crd.row;
        }
        if crd.row > max_row {
            max_row = crd.row;
        }
        if crd.col < min_col {
            min_col = crd.col;
        }
        if crd.col > max_col {
            max_col = crd.col;
        }
    }

    let mut num_connected_sides = 0;

    // loop from top to bottom to find connected sides on each level
    for row in min_row..=max_row {
        let mut all_tiles = Vec::new();
        for col in min_col..=max_col {
            let crd = Coordinate { row, col };

            let mut adder = true;
            if !rev {
                if crd.row > 0 {
                    let above = Coordinate {
                        row: crd.row - 1,
                        col: crd.col,
                    };
                    if coordinates.contains(&above) {
                        adder = false;
                    }
                }
            } else {
                let below = Coordinate {
                    row: crd.row + 1,
                    col: crd.col,
                };
                if coordinates.contains(&below) {
                    adder = false;
                }
            }

            if coordinates.contains(&crd) && adder {
                all_tiles.push(crd);
            }
        }
        if !all_tiles.is_empty() {
            num_connected_sides += 1;
            let mut cmp_col = all_tiles[0].col;
            for tl in all_tiles.iter().skip(1) {
                if tl.col != cmp_col + 1 {
                    num_connected_sides += 1;
                }
                cmp_col = tl.col;
            }
        }
    }

    num_connected_sides
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_p1() {
        let mut map = Map::new(DATA);
        map.find_all_connected();
        assert_eq!(map.price_p1(), 1930);
    }

    #[test]
    fn test_p1_2() {
        let data = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        let mut map = Map::new(data);
        map.find_all_connected();
        assert_eq!(map.price_p1(), 772);
    }

    #[test]
    fn test_p2() {
        let mut map = Map::new(DATA);
        map.find_all_connected();
        assert_eq!(map.calc_price_p2(), 1206);
    }

    #[test]
    fn test_p2_1() {
        let data = "AAAA
BBCD
BBCC
EEEC";
        let mut map = Map::new(data);
        map.find_all_connected();
        assert_eq!(map.calc_price_p2(), 80);
    }

    #[test]
    fn test_p2_2() {
        let data = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        let mut map = Map::new(data);
        map.find_all_connected();
        assert_eq!(map.calc_price_p2(), 236);
    }

    #[test]
    fn test_p2_3() {
        let data = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

        let mut map = Map::new(data);
        map.find_all_connected();
        assert_eq!(map.calc_price_p2(), 368);
    }

    #[test]
    fn test_p2_4() {
        let data = "AAAAAA
AABBAA
AABBAA
AACAAA
AAAAAA";
        let area_exp = 25;
        let sides_p2_exp = 10;
        let mut map = Map::new(data);
        map.find_all_connected();
        let price_exp = 10 * 25 + 4 * 4 + 4 * 1;
        assert_eq!(map.calc_price_p2(), price_exp);
    }

    #[test]
    fn test_p2_5() {
        let data = "BAA
AAA
AAA";

        let mut map = Map::new(data);
        map.find_all_connected();
        assert_eq!(map.calc_price_p2(), 4 * 1 + 6 * 8);
    }
}
