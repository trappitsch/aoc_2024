use std::collections::{HashMap, HashSet};
use std::slice::Iter;

fn main() {
    let data = std::fs::read_to_string("input").unwrap();
    let mut map = Map::new(&data);
    map.find_all_connected();
    let prices: u64 = map.plots.iter().map(|plot| plot.price()).sum();
    println!("Price part 1: {}", prices);

    println!("Price part 2: {}", map.calc_price_p2());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    row: usize,
    col: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum WalkDir {
    Left,
    Down,
    Right,
    Up,
}

impl WalkDir {
    fn try_next(&self) -> Iter<'_, WalkDir> {
        match &self {
            WalkDir::Right => [WalkDir::Up, WalkDir::Right, WalkDir::Down, WalkDir::Left].iter(),
            WalkDir::Down => [WalkDir::Right, WalkDir::Down, WalkDir::Left, WalkDir::Up].iter(),
            WalkDir::Left => [WalkDir::Down, WalkDir::Left, WalkDir::Up, WalkDir::Right].iter(),
            WalkDir::Up => [WalkDir::Left, WalkDir::Up, WalkDir::Right, WalkDir::Down].iter(),
        }
    }

    fn num_sides(&self, rhs: &WalkDir) -> u64 {
        let mut retval: i32 = 1;
        for (it, itval) in self.try_next().enumerate() {
            if itval == rhs {
                retval = match it {
                    1 => 0,
                    3 => 2,
                    _ => 1,
                };
                break;
            }
        }
        retval as u64
    }
}

#[derive(Debug, Clone)]
struct GardenPlot {
    name: char,
    coordinates: HashSet<Coordinate>,
    perimeter_coordinates: HashSet<Coordinate>,
    area: u64,
    perimeter: u64,
    perimeter_coordinates_p1: HashSet<Coordinate>,
    sides_p2: u64,
}

impl GardenPlot {
    fn new(name: char) -> Self {
        Self {
            name,
            coordinates: HashSet::new(),
            perimeter_coordinates: HashSet::new(),
            area: 0,
            perimeter: 0,
            perimeter_coordinates_p1: HashSet::new(),
            sides_p2: 0,
        }
    }

    fn add_plot(&mut self, rhs: GardenPlot) {
        self.coordinates.extend(&rhs.coordinates);
        self.area += rhs.area;
    }

    fn price(&self) -> u64 {
        self.area * self.perimeter
    }

    /// Walk the perimeter and count the turns => sides - 1
    fn walk_perimeter_count_turns(&mut self) {
        if self.coordinates.len() <= 2 {
            self.perimeter_coordinates.extend(&self.coordinates);
            self.sides_p2 = 4;
            return;
        };

        let mut turns_taken = 1; // the first side we move on comes without direction change.
        let start = self.find_top_left_corner();
        let mut current = start;
        let mut walking_direction = WalkDir::Right;

        self.perimeter_coordinates.insert(current);

        let field_below_current = Coordinate {
            row: current.row + 1,
            col: current.col,
        };
        //println!("{:?}", current);

        loop {
            for dir in walking_direction.try_next() {
                if let Some(next_spot) = next_tile_from_current(&current, dir) {
                    if self.coordinates.contains(&next_spot) {
                        //println!("{:?}", next_spot);
                        current = next_spot;
                        self.perimeter_coordinates.insert(next_spot);
                        if walking_direction != *dir {
                            turns_taken += walking_direction.num_sides(dir);
                        }
                        walking_direction = *dir;
                        break;
                    }
                };
            }

            if current == start {
                if self.coordinates.contains(&field_below_current)
                    && !self.perimeter_coordinates.contains(&field_below_current)
                {
                    walking_direction = WalkDir::Left;
                } else if walking_direction != WalkDir::Up {
                    turns_taken += 1;
                    break;
                } else {
                    break;
                }
            }
        }

        self.sides_p2 = turns_taken;
    }

    /// Find out if the given area is inside this area by counting perimeters to each side.
    fn contains(&self, inside: &GardenPlot) -> bool {
        for cin in inside.coordinates.iter() {
            let peris_left = self
                .perimeter_coordinates
                .iter()
                .filter(|this| {
                    let (row, col) = (this.row, this.col);
                    cin.row == row && cin.col > col
                })
                .count();
            let peris_right = self
                .perimeter_coordinates
                .iter()
                .filter(|this| {
                    let (row, col) = (this.row, this.col);
                    cin.row == row && cin.col < col
                })
                .count();
            let peris_top = self
                .perimeter_coordinates
                .iter()
                .filter(|this| {
                    let (row, col) = (this.row, this.col);
                    cin.row < row && cin.col == col
                })
                .count();
            let peris_botton = self
                .perimeter_coordinates
                .iter()
                .filter(|this| {
                    let (row, col) = (this.row, this.col);
                    cin.row > row && cin.col == col
                })
                .count();

            if peris_left % 2 == 0
                || peris_right % 2 == 0
                || peris_top % 2 == 0
                || peris_botton % 2 == 0
            {
                return false;
            }
        }
        true
    }

    /// Find out if a second area is touching this one.
    fn is_touching(&self, rhs: &GardenPlot) -> bool {
        for crd in rhs.coordinates.iter() {
            if crd.row > 0
                && self.coordinates.contains(&Coordinate {
                    row: crd.row - 1,
                    col: crd.col,
                })
            {
                return true;
            }
            if self.coordinates.contains(&Coordinate {
                row: crd.row + 1,
                col: crd.col,
            }) {
                return true;
            }
            if crd.col > 0
                && self.coordinates.contains(&Coordinate {
                    row: crd.row,
                    col: crd.col - 1,
                })
            {
                return true;
            }
            if self.coordinates.contains(&Coordinate {
                row: crd.row,
                col: crd.col + 1,
            }) {
                return true;
            }
        }
        false
    }

    fn find_top_left_corner(&self) -> Coordinate {
        let mut top_left = Coordinate {
            row: usize::MAX,
            col: usize::MAX,
        };
        for crd in self.coordinates.iter() {
            if crd.row < top_left.row || (crd.row == top_left.row && crd.col < top_left.col) {
                top_left = *crd;
            }
        }
        top_left
    }
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

    /// Find number of sides for part 2 for all plots
    fn find_sides_p2(&mut self) -> Vec<u64> {
        let mut sides = Vec::new();

        self.plots
            .iter_mut()
            .for_each(|plt| plt.walk_perimeter_count_turns());

        for (pt, plt) in self.plots.iter().enumerate() {
            let mut side = plt.sides_p2;

            // now go through all other plots and check if they are inside this one.
            let mut inside_plots = Vec::new();
            for (pt2, plt2) in self.plots.iter().enumerate() {
                if pt == pt2 {
                    continue;
                } else if plt.contains(plt2) {
                    inside_plots.push(plt2.clone());
                }
            }


            // Now check if inside plots are touching each other.
            let mut combined_inside_plots = Vec::new();
            while !inside_plots.is_empty() {
                let mut combine_vec = Vec::new();
                let mut combined_plot = inside_plots.remove(0);

                for (it, plts) in inside_plots.iter().enumerate() {
                    if combined_plot.is_touching(plts) {
                        combine_vec.push(it);
                    }
                }

                if combine_vec.is_empty() || inside_plots.is_empty() {
                    combined_inside_plots.push(combined_plot);
                } else {
                    for it in combine_vec {
                        let add_plot = inside_plots.remove(it);
                        combined_plot.add_plot(add_plot);
                    }
                    if inside_plots.is_empty() {
                        combined_inside_plots.push(combined_plot);
                    }
                }
            }

            combined_inside_plots.iter_mut().for_each(|plt| plt.walk_perimeter_count_turns());
            side += combined_inside_plots.iter().map(|plt| plt.sides_p2).sum::<u64>();

            sides.push(side);
        }

        sides
    }

    fn calc_price_p2(&mut self) -> u64 {
        let mut price = 0;
        let sides = self.find_sides_p2();
        for (pt, plt) in self.plots.iter().enumerate() {
            //println!(
            //    "Char: {}, sides: {}, area: {}",
            //    plt.name, sides[pt], plt.area
            //);
            price += plt.area * sides[pt];
        }
        price
    }

    /// Loop through all existing coordinates and return one that is not yet in list
    fn find_next_starting_coordinate(&mut self) -> Option<Coordinate> {
        let mut visited: HashSet<Coordinate> = HashSet::new();
        for plt in self.plots.iter() {
            visited.extend(&plt.coordinates);
        }
        for row in 0..self.data.len() {
            for col in 0..self.data[row].len() {
                let coord_to_test = Coordinate { row, col };
                if !visited.contains(&coord_to_test) {
                    return Some(coord_to_test);
                }
            }
        }
        None
    }

    /// Find all connected regions and add them to the plots vector.
    fn find_all_connected(&mut self) {
        while let Some(start_coord) = self.find_next_starting_coordinate() {
            let connected = self.find_connected(start_coord);
            let area = connected.len() as u64;
            let mut perimeter = 0;
            let mut perimeter_coordinates_p1 = HashSet::new();
            for crd in connected.iter() {
                let peri = self.perimeter(*crd);
                if peri > 0 {
                    perimeter_coordinates_p1.insert(*crd);
                }
                perimeter += peri;
            }

            let plot = GardenPlot {
                name: self.get_char(start_coord),
                coordinates: connected,
                perimeter_coordinates: HashSet::new(),
                area,
                perimeter,
                perimeter_coordinates_p1,
                sides_p2: 0,
            };

            self.plots.push(plot);
        }
    }

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
        let (row, col) = (coord.row, coord.col);

        if row > 0 && self.data[row - 1][col] == ch {
            adjacents.insert(Coordinate { row: row - 1, col });
        }
        if row < self.data.len() - 1 && self.data[row + 1][col] == ch {
            adjacents.insert(Coordinate { row: row + 1, col });
        }
        if col > 0 && self.data[row][col - 1] == ch {
            adjacents.insert(Coordinate { row, col: col - 1 });
        }
        if col < self.data[row].len() - 1 && self.data[row][col + 1] == ch {
            adjacents.insert(Coordinate { row, col: col + 1 });
        }
        adjacents
    }

    /// Get a character at a given coordinate.
    fn get_char(&self, coord: Coordinate) -> char {
        self.data[coord.row][coord.col]
    }

    /// Get the perimeter around a given point, 0 if the same, otherwise 1.
    fn perimeter(&self, coord: Coordinate) -> u64 {
        let (row, col) = (coord.row, coord.col);
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
}

/// Get coordinates for next tile from current in walking direction.
/// If negative in any direction, return None.
fn next_tile_from_current(curr: &Coordinate, dir: &WalkDir) -> Option<Coordinate> {
    let row = curr.row;
    let col = curr.col;

    match dir {
        WalkDir::Right => Some(Coordinate { row, col: col + 1 }),
        WalkDir::Down => Some(Coordinate { row: row + 1, col }),
        WalkDir::Left => {
            if col > 0 {
                Some(Coordinate { row, col: col - 1 })
            } else {
                None
            }
        }
        WalkDir::Up => {
            if row > 0 {
                Some(Coordinate { row: row - 1, col })
            } else {
                None
            }
        }
    }
}

fn number_connected_sides_top_bottom(perimeter: &HashSet<Coordinate>) -> u64 {
    let mut min_row = 0;
    let mut max_row = usize::MAX;
    let mut min_col = 0;
    let mut max_col = usize::MAX;

    for crd in perimeter.iter() {
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
            if crd.row > 0 {
                let above = Coordinate {
                    row: crd.row - 1,
                    col: crd.col,
                };
                if perimeter.contains(&above) {
                    adder = false;
                }
            }

            if perimeter.contains(&crd) && adder {
                all_tiles.push(crd);
            }
        }
        if all_tiles.len() != 1 {
            num_connected_sides += 1;
            let mut cmp_col= all_tiles[0].col;
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
        let prices: u64 = map.plots.iter().map(|plot| plot.price()).sum();
        assert_eq!(prices, 1930);
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
        let prices: u64 = map.plots.iter().map(|plot| plot.price()).sum();
        assert_eq!(prices, 772);
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
        let sides = map.find_sides_p2();

        assert_eq!(map.plots[0].area, area_exp);
        assert_eq!(sides[0], sides_p2_exp);

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
