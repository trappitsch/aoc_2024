use std::collections::HashSet;

fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    let topo = TopoMap::new(&s);
    println!("Part 1: {}", topo.sum_trailhead_scores());
    println!("Part 2: {}", topo.sum_trailhead_ratings());
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    /// Find all horizontal and vertical neighbors of a coordinate if within grid.
    fn find_neighbors(&self, max_row: usize, max_col: usize) -> Vec<Coord> {
        let mut neighbors = Vec::new();

        if self.row > 0 {
            neighbors.push(Coord {
                row: self.row - 1,
                col: self.col,
            });
        }
        if self.row < max_row {
            neighbors.push(Coord {
                row: self.row + 1,
                col: self.col,
            });
        }
        if self.col > 0 {
            neighbors.push(Coord {
                row: self.row,
                col: self.col - 1,
            });
        }
        if self.col < max_col {
            neighbors.push(Coord {
                row: self.row,
                col: self.col + 1,
            });
        }

        neighbors
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Trail {
    trail: [Coord; 10],
    current_index: usize,
    insert_index: usize,
}

impl Trail {
    fn new() -> Self {
        Self {
            trail: [Coord { row: 0, col: 0 }; 10],
            current_index: 0,
            insert_index: 0,
        }
    }

    fn new_with_coordinate(coord: Coord) -> Self {
        let mut tmp = Self::new();
        tmp.add_coord(coord);
        tmp
    }

    fn add_coord(&mut self, coord: Coord) {
        self.current_index = self.insert_index;
        self.trail[self.insert_index] = coord;
        self.insert_index += 1;
    }

    fn get_current_coord(&self) -> Coord {
        self.trail[self.current_index]
    }
}

struct TopoMap {
    altitude: Vec<Vec<usize>>,
    end: usize,
    delta: usize,
}

impl TopoMap {
    fn new(s: &str) -> Self {
        let end = 9;
        let delta = 1;

        let mut altitude = Vec::new();
        for line in s.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c.to_digit(10).unwrap() as usize);
            }
            altitude.push(row);
        }

        Self {
            altitude,
            end,
            delta,
        }
    }

    fn sum_trailhead_ratings(&self) -> usize {
        let trailheads = self.find_all_trails();
        let mut count = 0;
        for t in trailheads {
            count += t.len();
        }
        count
    }

    /// For each trailhead and the associated trails, count how many different ends can be reached.
    /// Then sum up the counts of all trailheads.
    fn sum_trailhead_scores(&self) -> usize {
        let trailheads = self.find_all_trails();
        let mut count = 0;

        for trails in trailheads {
            let mut ends = HashSet::new();
            for trail in trails {
                ends.insert(trail.get_current_coord());
            }
            count += ends.len();
        }
        count
    }

    fn find_all_trails(&self) -> Vec<HashSet<Trail>> {
        // max index of row and col
        let max_row = self.altitude.len() - 1;
        let max_col = self.altitude[0].len() - 1;

        let mut trailheads: Vec<HashSet<Trail>> = Vec::new();

        // find all 0 in the grid and add to coords
        for row in 0..max_row + 1 {
            for col in 0..max_col + 1 {
                if self.altitude[row][col] == 0 {
                    let coord = Coord { row, col };
                    let trail = Trail::new_with_coordinate(coord);
                    let mut these_trails = HashSet::new();
                    these_trails.insert(trail);
                    trailheads.push(these_trails);
                }
            }
        }

        let mut new_trailheads: Vec<HashSet<Trail>> = Vec::new();
        for trails in trailheads {
            let mut find_next = 1;
            let mut these_trails = trails.clone();
            while find_next <= self.end {
                let mut new_trails = HashSet::new();
                for trail in &these_trails {
                    let current_coord = trail.get_current_coord();
                    let neighbors = current_coord.find_neighbors(max_row, max_col);
                    for neighbor in neighbors {
                        if self.altitude[neighbor.row][neighbor.col] == find_next {
                            let mut new_trail = *trail;
                            new_trail.add_coord(neighbor);
                            new_trails.insert(new_trail);
                        }
                    }
                }
                these_trails = new_trails;
                find_next += self.delta;
            }
            new_trailheads.push(these_trails);
        }

        new_trailheads
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOPO: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_topo_map() {
        let topo = TopoMap::new(TOPO);
        assert_eq!(topo.altitude[0][0], 8);
        assert_eq!(topo.altitude[7][7], 2);
    }

    #[test]
    fn test_hashset_trails() {
        let mut trails: HashSet<Trail> = HashSet::new();
        let mut trail = Trail::new_with_coordinate(Coord { row: 1, col: 2 });
        let mut trail2 = trail;
        let mut trail3 = trail;
        trail.add_coord(Coord { row: 2, col: 3 });
        trail2.add_coord(Coord { row: 3, col: 4 });
        trail3.add_coord(Coord { row: 2, col: 3 });

        trails.insert(trail);
        trails.insert(trail2);
        trails.insert(trail3);

        assert_eq!(trails.len(), 2);
    }

    #[test]
    fn test_p1() {
        let topo = TopoMap::new(TOPO);
        assert_eq!(topo.sum_trailhead_scores(), 36);
    }

    #[test]
    fn test_p2() {
        let topo = TopoMap::new(TOPO);
        assert_eq!(topo.sum_trailhead_ratings(), 81);
    }
}
