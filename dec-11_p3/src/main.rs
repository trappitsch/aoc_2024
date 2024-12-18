//! THIS IS ALL FUCKING BULLSHIT TOO...

fn main() {
    //println!("{}", numbers_at_max_level(1, 6));
    println!("{:?}", SingleDigitNode::new(8));
}

fn numbers_at_max_level(value: u64, max_level: u64) -> u64 {
    let lt = LookupTableSingleNodes::new();
    let mut levels = Vec::new();
    let mut values = Vec::new();

    values.push(vec![value]);
    levels.push(vec![0]);

    loop {
        let mut this_level = Vec::new();
        let mut this_values = Vec::new();

        println!("{:?}", levels.last().unwrap());
        println!("{:?}", values.last().unwrap());

        for (it, &val) in values.last().unwrap().iter().enumerate() {
            let curr_level = levels.last().unwrap()[it];
            if curr_level == max_level {
                this_level.push(curr_level);
                this_values.push(val);
            } else if !is_single_digit(val) {
                let (left, right) = calc_next(val);
                this_values.push(left.unwrap());
                this_level.push(curr_level + 1);
                if let Some(right) = right {
                    this_values.push(right);
                    this_level.push(curr_level + 1);
                }

            } else {
                let node = &lt.nodes[val as usize];
                if curr_level + node.levels > max_level {
                    let l2p = max_level - curr_level;
                    for val in node.values[l2p as usize].iter() {
                        this_level.push(max_level);
                        this_values.push(*val);
                    }
                } else {
                    for val in node.values.last().unwrap().iter() {
                        this_level.push(curr_level + node.levels);
                        this_values.push(*val);
                    }
                }
            }
        }
        levels.push(this_level);
        values.push(this_values);

        if levels.last().unwrap().iter().all(|x| *x == max_level) {
            break;
        }
    }

    values.last().unwrap().len() as u64
}

#[derive(Debug)]
struct SingleDigitNode {
    value: u64,
    levels: u64,           // How many times can we blink until all single digits again
    values: Vec<Vec<u64>>, // All possible values for this node for 0 < n <= levels levels
}

struct LookupTableSingleNodes {
    nodes: [SingleDigitNode; 10],
}

impl LookupTableSingleNodes {
    fn new() -> LookupTableSingleNodes {
        let nodes = [
            SingleDigitNode::new(0),
            SingleDigitNode::new(1),
            SingleDigitNode::new(2),
            SingleDigitNode::new(3),
            SingleDigitNode::new(4),
            SingleDigitNode::new(5),
            SingleDigitNode::new(6),
            SingleDigitNode::new(7),
            SingleDigitNode::new(8),
            SingleDigitNode::new(9),
        ];

        LookupTableSingleNodes { nodes }
    }

    fn get_max_levels(&self, digit: usize) -> u64 {
        self.nodes[digit].levels
    }

    fn get_max_values(&self, digit: usize) -> &Vec<Vec<u64>> {
        &self.nodes[digit].values
    }

    fn get_num_values(&self, digit: usize, level: usize) -> usize {
        if level > self.nodes[digit].levels as usize {
            panic!("Level out of bounds");
        }
        self.nodes[digit].values[level].len()
    }
}

impl SingleDigitNode {
    fn new(value: u64) -> SingleDigitNode {
        let mut levels = 0;

        let mut values = Vec::new();
        let mut current_values = vec![value];
        loop {
            let mut next_values = Vec::new();
            for val in &current_values {
                let (left, right) = calc_next(*val);
                next_values.push(left.unwrap());
                if let Some(right) = right {
                    next_values.push(right);
                }
            }
            levels += 1;
            let continue_condition = next_values.iter().any(|&x| !is_single_digit(x));

            values.push(next_values.clone());
            current_values = next_values;
            if !continue_condition {
                break;
            }

            println!("{:?}", current_values);

            if levels > 7 {
                break;
            }
        }

        SingleDigitNode {
            value,
            levels,
            values,
        }
    }
}

fn calc_next(value: u64) -> (Option<u64>, Option<u64>) {
    if value == 0 {
        return (Some(1), None);
    } else {
        let num_digs = value.ilog10() + 1;
        if num_digs % 2 == 0 {
            let left = value / 10u64.pow(num_digs / 2);
            let right = value % 10u64.pow(num_digs / 2);
            (Some(left), Some(right))
        } else {
            (Some(value * 2024), None)
        }
    }
}

fn is_single_digit(value: u64) -> bool {
    if value == 0 {
        return true;
    }
    value.ilog10() == 0
}
