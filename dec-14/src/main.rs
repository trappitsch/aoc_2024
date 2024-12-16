fn main() {
    let data = std::fs::read_to_string("input").unwrap();
    let mut map = Map::new_from_data(&data, 101, 103);
    map.move_robots(100);
    println!("Part 1: {}", map.count_robots_quadrants_mult());

    // Tree is at frame 6512 - brute forced by observing...
    let mut map = MapP2::new_from_data(&data);
    for _ in 0..10000 {
        map.next_step();
        map.update_map();
        if map.steps == 6512 {
            println!("{}", map);
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn new(x: i64, y: i64) -> Coord {
        Coord { x, y }
    }
}

// implement operations for Coord
impl std::ops::Add for Coord {
    type Output = Coord;

    fn add(self, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::RemAssign for Coord {
    fn rem_assign(&mut self, other: Coord) {
        self.x = self.x.rem_euclid(other.x);
        self.y = self.y.rem_euclid(other.y);
    }
}

// implement scalar multiplication with u64 for Coord
impl std::ops::Add<i64> for Coord {
    type Output = Coord;

    fn add(self, scalar: i64) -> Coord {
        Coord {
            x: self.x + scalar,
            y: self.y + scalar,
        }
    }
}

impl std::ops::Mul<i64> for Coord {
    type Output = Coord;

    fn mul(self, scalar: i64) -> Coord {
        Coord {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl std::ops::Div<i64> for Coord {
    type Output = Coord;

    fn div(self, scalar: i64) -> Coord {
        Coord {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

struct Robot {
    position: Coord,
    velocity: Coord,
}

impl Robot {
    fn new_from_line(s: &str) -> Robot {
        let parts = s.trim().split(" ").collect::<Vec<&str>>();
        let pos_str = parts[0][2..].split(",").collect::<Vec<&str>>();
        let vel_str = parts[1][2..].split(",").collect::<Vec<&str>>();

        let pos = Coord::new(
            pos_str[0].parse::<i64>().unwrap(),
            pos_str[1].parse::<i64>().unwrap(),
        );
        let vel = Coord::new(
            vel_str[0].parse::<i64>().unwrap(),
            vel_str[1].parse::<i64>().unwrap(),
        );

        Robot {
            position: pos,
            velocity: vel,
        }
    }

    fn move_robot(&mut self, time: i64) {
        self.position = self.position + self.velocity * time;
    }
}

struct Map {
    robots: Vec<Robot>,
    size: Coord,
}

impl Map {
    fn new_from_data(data: &str, width: i64, height: i64) -> Map {
        let mut robots = Vec::new();
        for line in data.lines() {
            robots.push(Robot::new_from_line(line));
        }

        Map {
            robots,
            size: Coord::new(width, height),
        }
    }

    fn move_robots(&mut self, time: i64) {
        for robot in self.robots.iter_mut() {
            robot.move_robot(time);
            robot.position %= self.size;
        }
    }

    fn count_robots_in_area(&self, min_xy: Coord, max_xy: Coord) -> usize {
        let mut count = 0;
        for robot in self.robots.iter() {
            if robot.position.x >= min_xy.x
                && robot.position.x < max_xy.x
                && robot.position.y >= min_xy.y
                && robot.position.y < max_xy.y
            {
                count += 1;
            }
        }
        count
    }

    fn count_robots_quadrants_mult(&self) -> usize {
        let center_x = self.size.x / 2;
        let center_y = self.size.y / 2;

        let q0 = [Coord::new(0, 0), self.size / 2];
        let q1 = [
            Coord::new(center_x + 1, 0),
            Coord::new(self.size.x, center_y),
        ];
        let q2 = [
            Coord::new(0, center_y + 1),
            Coord::new(center_x, self.size.y),
        ];
        let q3 = [self.size / 2 + 1, self.size];

        let quadrants = [q0, q1, q2, q3];

        for (n, q) in quadrants.iter().enumerate() {
            println!("Quadrant: {n}");
            println!("xmin {} ymin {}", q[0].x, q[0].y);
            println!("xmax {} ymax {}", q[1].x, q[1].y);
            println!("# robots: {}", self.count_robots_in_area(q[0], q[1]));
        }

        quadrants
            .iter()
            .map(|q| self.count_robots_in_area(q[0], q[1]))
            .product::<usize>()
    }
}

struct MapP2 {
    robots: Vec<Robot>,
    map: [[u64; 101]; 103],
    steps: u64,
}

impl MapP2 {
    fn new_from_data(data: &str) -> MapP2 {
        let mut robots = Vec::new();
        for line in data.lines() {
            robots.push(Robot::new_from_line(line));
        }
        let map = [[0; 101]; 103];

        MapP2 {
            robots,
            map,
            steps: 0,
        }
    }

    fn next_step(&mut self) {
        self.robots.iter_mut().for_each(|r| {
            r.move_robot(1);
            r.position %= Coord::new(101, 103);
        });
        self.update_map();
        self.steps += 1;
    }

    fn update_map(&mut self) {
        self.map.iter_mut().for_each(|m| *m = [0; 101]);
        for robot in self.robots.iter() {
            self.map[robot.position.y as usize][robot.position.x as usize] += 1;
        }
    }
}

impl std::fmt::Display for MapP2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.map.iter() {
            for cell in row.iter() {
                if *cell > 0 {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "Steps: {}", self.steps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_data_to_robot() {
        for line in DATA.lines() {
            let _ = Robot::new_from_line(line);
        }
    }

    #[test]
    fn test_p1() {
        let mut map = Map::new_from_data(DATA, 11, 7);
        map.move_robots(100);
        assert_eq!(map.count_robots_quadrants_mult(), 12);
    }
}
