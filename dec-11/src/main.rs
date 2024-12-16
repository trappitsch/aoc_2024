fn main() {
    let data = std::fs::read_to_string("input").unwrap();
    let data: Vec<u64> = data.trim().split(" ").map(|x| x.parse().unwrap()).collect();
    let mut arr = Arrangement::new(data.clone());

    arr.blink_n(25);

    println!("Part 1: {}", arr.get_number_stones());

    let mut arr = Arrangement::new(data);
    //arr.blink_n(75);  // FIXME: takes way too long
    println!("Part 2: {}", arr.get_number_stones());
}

struct Stone2 {
    value: u64,
    sum_stones: u64,
    blinks: u64,
}

struct Arrangement {
    row: Vec<Stone>,
    blinks: u64,
}


impl Arrangement {
    fn new(values: Vec<u64>) -> Arrangement {
        let mut row = Vec::new();
        for st in values {
            row.push(Stone::new(st))
        }

        Arrangement { row, blinks: 0 }
    }

    // Blink multiple times 
    fn blink_n(&mut self, n: u64) {
        for blk in 0..n {
            self.blink()
        }
    }

    /// Blink once
    fn blink(&mut self) {
        let mut new_row = Vec::new();
        for stone in &self.row {
            stone.split().iter().for_each(|x| new_row.push(*x));
        }
        self.row = new_row;
        self.blinks += 1;
    }

    fn get_number_stones(&self) -> usize {
        self.row.len()
    }

    fn get_values(&self) -> Vec<u64> {
        self.row.iter().map(|x| x.value).collect()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Stone {
    value: u64,
}

impl Stone {
    fn new(value: u64) -> Stone {
        Stone { value }
    }

    fn split(&self) -> Vec<Stone> {
        match self.value {
            0 => vec![Stone::new(1)],
            _ => match self.number_digits() % 2 {
                0 => {
                    let parts = self.split_digits();
                    vec![Stone::new(parts[0]), Stone::new(parts[1])]
                }
                _ => vec![Stone::new(self.value * 2024)],
            },
        }
    }

    fn number_digits(&self) -> u64 {
        if self.value == 0 {
            return 1;
        }
        self.value.ilog10() as u64 + 1
    }

    fn split_digits(&self) -> [u64; 2] {
        let dstr = format!("{}", self.value);
        let p1 = dstr[..dstr.len() / 2].parse::<u64>().unwrap();
        let p2 = dstr[dstr.len() / 2..].parse::<u64>().unwrap();
        [p1, p2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_digits() {
        let d3 = Stone::new(123);
        let d1 = Stone::new(1);
        let d1_0 = Stone::new(0);
        let d7 = Stone::new(1234567);
        assert_eq!(d3.number_digits(), 3);
        assert_eq!(d1.number_digits(), 1);
        assert_eq!(d1_0.number_digits(), 1);
        assert_eq!(d7.number_digits(), 7);
    }

    #[test]
    fn test_split_digits() {
        let stone = Stone::new(1234);
        assert_eq!(stone.split_digits(), [12, 34]);
        let stone = Stone::new(19);
        assert_eq!(stone.split_digits(), [1, 9]);
        let stone = Stone::new(9816274900000000);
        assert_eq!(stone.split_digits(), [98162749, 0]);
    }

    #[test]
    fn test_stone_split() {
        let stone = Stone::new(0);
        let exp = vec![Stone::new(1)];
        assert_eq!(stone.split(), exp);
        let stone = Stone::new(1234);
        let exp = vec![Stone::new(12), Stone::new(34)];
        assert_eq!(stone.split(), exp);
        let stone = Stone::new(3);
        let exp = vec![Stone::new(2024 * 3)];
        assert_eq!(stone.split(), exp);
    }

    #[test]
    fn test_p1_blink() {
        let mut arr = Arrangement::new(vec![125, 17]);
        arr.blink();
        assert_eq!(arr.get_values(), vec![253000, 1, 7]);
    }

    #[test]
    fn test_p1_blinks() {
        let mut arr = Arrangement::new(vec![125, 17]);
        arr.blink_n(6);
        assert_eq!(arr.get_number_stones(), 22);
    }
}
