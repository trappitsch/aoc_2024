const PRICE_OFFSET: i64 = 10_000_000_000_000;

#[derive(Debug)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn new_from_target_string(s: &str) -> Coord {
        let parts = s.split(": ").collect::<Vec<&str>>();
        let coords = parts[1].split_whitespace().collect::<Vec<&str>>();
        let x = coords[0][2..coords[0].len() - 1].parse::<i64>().unwrap();
        let y = coords[1][2..coords[1].len()].parse::<i64>().unwrap();
        Coord { x, y }
    }
}

#[derive(Debug)]
enum Button {
    A(Coord, i64),
    B(Coord, i64),
}

impl Button {
    fn new_from_str(s: &str) -> Button {
        if !s.starts_with("Button") {
            panic!("Invalid input");
        };
        let parts = s.trim().split(": ").collect::<Vec<&str>>();
        let coords = parts[1].split_whitespace().collect::<Vec<&str>>();
        let x = coords[0][1..coords[0].len() - 1].parse::<i64>().unwrap();
        let y = coords[1][1..coords[1].len()].parse::<i64>().unwrap();

        if parts[0].contains("A") {
            Button::A(Coord { x, y }, 3)
        } else {
            Button::B(Coord { x, y }, 1)
        }
    }

    fn dx(&self) -> i64 {
        match self {
            Button::A(coord, _) => coord.x,
            Button::B(coord, _) => coord.x,
        }
    }

    fn dy(&self) -> i64 {
        match self {
            Button::A(coord, _) => coord.y,
            Button::B(coord, _) => coord.y,
        }
    }

    fn price(&self) -> i64 {
        match self {
            Button::A(_, price) => *price,
            Button::B(_, price) => *price,
        }
    }
}

#[derive(Debug)]
struct ClawMachine {
    btn_a: Button,
    btn_b: Button,
    target: Coord,
}

impl ClawMachine {
    fn new_w_btn(btn_a: Button, btn_b: Button, target: Coord) -> ClawMachine {
        ClawMachine { btn_a, btn_b, target }
    }

    /// Returns None if no solution exist, otherwise clicks button A and B as array
    fn solution(&self) -> Option<[i64; 2]> {
        let nb_nominator = self.target.y * self.btn_a.dx() - self.target.x * self.btn_a.dy();
        let nb_denominator = self.btn_b.dy() * self.btn_a.dx() - self.btn_b.dx() * self.btn_a.dy();

        if nb_nominator % nb_denominator != 0 {
            return None;
        }

        let nb = nb_nominator / nb_denominator;

        let na_nominator = self.target.x - nb * self.btn_b.dx();
        let na_denominator = self.btn_a.dx();

        if na_nominator % na_denominator != 0 {
            return None;
        }

        let na = na_nominator / na_denominator;
        Some([na, nb])
    }

    /// Returns None if no solution exist, otherwise total price for all clicks.
    fn price(&self) -> Option<i64> {
        self.solution()
            .map(|[na, nb]| na * self.btn_a.price() + nb * self.btn_b.price())
    }
}

struct AllClawMachines {
    machines: Vec<ClawMachine>,
}

impl AllClawMachines {
    fn new(s: &str) -> AllClawMachines {
        let mut machines = Vec::new();

        for (lt, line) in s.lines().step_by(4).enumerate() {
            let btn_a = Button::new_from_str(line);
            let btn_b = Button::new_from_str(s.lines().nth(lt * 4 + 1).unwrap());
            let target = Coord::new_from_target_string(s.lines().nth(lt * 4 + 2).unwrap());

            machines.push(ClawMachine::new_w_btn(btn_a, btn_b, target));
        }

        AllClawMachines { machines }
    }

    fn total_price(&self) -> i64 {
        self.machines.iter().map(|m| m.price().unwrap_or_default()).sum()
    }
}

fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    let mut machines = AllClawMachines::new(&s);
    println!("Price part 1: {}", machines.total_price());

    for mach in machines.machines.iter_mut() {
        mach.target.x += PRICE_OFFSET;
        mach.target.y += PRICE_OFFSET;
    }
    println!("Price part 2: {}", machines.total_price());
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_p1() {
        let machines = AllClawMachines::new(DATA);
        assert_eq!(machines.total_price(), 480);
    }
}
