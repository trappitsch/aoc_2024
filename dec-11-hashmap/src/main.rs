use std::collections::HashMap;

fn main() {
    let d = std::fs::read_to_string("input").unwrap();
    let d = d.split_whitespace().collect::<Vec<&str>>();
    let data = d.iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    println!("data: {:?}", data);
    let mut walk = Walk::new(vec![0]);
    //let mut walk = Walk::new(data);
    let depth = 75;
    let total = walk.walk_to_depth(depth);
    println!("Total nodes at depth {}: {}", depth, total);
    //println!("Values: {:?}", walk.values);
}

struct Walk {
    pre_computed: PreComputed,
    values: Vec<u64>,
    levels: Vec<u64>,
}

impl Walk {
    fn new(value: Vec<u64>) -> Walk {
        let mut pc = PreComputed::new();
        pc.add_defaults();
        let values = value;
        let levels = vec![0; values.len()]; 

        Walk {
            pre_computed: pc,
            values,
            levels,
        }
    }

    /// Walk to a specific depth and count the number of total nodes at depth
    fn walk_to_depth(&mut self, depth: u64) -> u64 {
        loop {
            let mut new_values = Vec::new();
            let mut new_levels = Vec::new();
            for (it, val) in self.values.iter().enumerate() {
                // already there
                if self.levels[it] == depth {
                    new_values.push(*val);
                    new_levels.push(depth);
                    continue;
                }
                let mut node = self.pre_computed.get_node(*val);

                // so we are close - move in steps of one
                if self.levels[it] + node.depth > depth {
                    node = Node::new(*val, 1);
                }

                // now add the children to the new values
                node.child_values.iter().for_each(|x| {
                    new_values.push(*x);
                    new_levels.push(self.levels[it] + node.depth);
                });
            }

            self.values = new_values;
            self.levels = new_levels;

            // print min, max level
            let min = self.levels.iter().min().unwrap();
            let max = self.levels.iter().max().unwrap();
            println!("Min: {}, Max: {}", min, max);

            if self.levels.iter().all(|x| *x == depth) {
                break;
            }
        }

        self.values.len() as u64
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    value: u64,
    depth: u64,
    child_values: Vec<u64>,
}

impl Node {
    fn new(value: u64, depth: u64) -> Node {
        let mut child_values = vec![value];
        for _ in 0..depth {
            let mut new_vec = Vec::new();
            for v in &child_values {
                let next = calc_next(*v);
                next.iter().for_each(|x| {
                    if let Some(val) = x {
                        new_vec.push(*val);
                    }
                });
            }
            child_values = new_vec;
        }

        Node {
            value,
            depth,
            child_values,
        }
    }
}

struct PreComputed {
    cache: HashMap<u64, Node>,
}

impl PreComputed {
    fn new() -> PreComputed {
        PreComputed {
            cache: HashMap::new(),
        }
    }

    fn add_defaults(&mut self) {
        self.add_by_value(0, 4);
        self.add_by_value(1, 3);
        self.add_by_value(2, 3);
        self.add_by_value(3, 3);
        self.add_by_value(4, 3);
        self.add_by_value(5, 5);
        self.add_by_value(6, 5);
        self.add_by_value(7, 5);
        self.add_by_value(8, 4);
        self.add_by_value(9, 5);
        self.add_by_value(26, 1);
        self.add_by_value(32, 1);
        self.add_by_value(77, 1);
    }

    fn add_by_value(&mut self, value: u64, depth: u64) {
        let node = Node::new(value, depth);
        self.cache.insert(value, node);
    }

    /// Get a node from the hashmap if it exists, otherwise create and add it
    fn get_node(&mut self, value: u64) -> Node {
        if let Some(node) = self.cache.get(&value) {
            node.clone()
        } else {
            let node = Node::new(value, 1);
            self.cache.insert(value, node.clone());
            node
        }
    }
}

fn calc_next(value: u64) -> [Option<u64>; 2] {
    if value == 0 {
        return [Some(1), None];
    }

    let num_digits = value.ilog10() + 1;

    if num_digits % 2 == 1 {
        [Some(value * 2024), None]
    } else {
        let left = value / 10u64.pow(num_digits / 2);
        let right = value % 10u64.pow(num_digits / 2);
        [Some(left), Some(right)]
    }
}
