//! Do this puzzle with tree traversal recursively, keep sum of tree.
use std::collections::HashMap;
use std::sync::{
    {Arc, Mutex},
};
use std::thread;

fn main() {
    let max_level = 75;
    
    let inp = std::fs::read_to_string("input").unwrap();
    let values: Vec<u64> = inp.trim().split(" ").map(|x| x.parse().unwrap()).collect();



    let total_sum = Arc::new(Mutex::new(0_u64));
    let mut handles = Vec::new();

    let now = std::time::Instant::now();

    for val in values {
        let total_sum = total_sum.clone();
        let hndl = thread::spawn(move || {
            let mut tree = Tree::new(val, max_level);
            tree.walk_through_tree();
            println!("Value: {} done, sum: {}", val, tree.sum_max_reached);
            loop {
                if let Ok(mut sum) = total_sum.try_lock() {
                    *sum += tree.sum_max_reached;
                    break;
                }
            }
        });
        handles.push(hndl);
    }

    for hndl in handles {
        hndl.join().unwrap();
    }

    let elapsed_time = now.elapsed().as_millis() as f64 / 1000.0;
    let tsum = *total_sum.lock().unwrap();
    println!("Max level: {}, sum: {}, seconds elapsed {}", max_level, tsum, elapsed_time);
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum NodeGoNext {
    DownLeft,
    DownRight,
    Up,
    Done,
}

impl NodeGoNext {
    fn next(&self) -> Self {
        match self {
            NodeGoNext::DownLeft => NodeGoNext::DownRight,
            NodeGoNext::DownRight => NodeGoNext::Up,
            NodeGoNext::Up => NodeGoNext::Done,
            NodeGoNext::Done => panic!("You are done and didn't notice!"),
        }
    }
}

struct Tree {
    value: u64,
    max_level: u64,
    sum_max_reached: u64,
    state: NodeGoNext,
    level: u64,
    previous_values: Vec<u64>,
    previous_states: Vec<NodeGoNext>,
}

impl Tree {
    fn new(value: u64, max_level: u64) -> Self {
        Tree {
            value,
            max_level,
            sum_max_reached: 0,
            state: NodeGoNext::DownLeft,
            level: 0,
            previous_values: Vec::new(),
            previous_states: Vec::new(),
        }
    }

    fn walk_through_tree(&mut self) {
        while self.state != NodeGoNext::Done {
            self.next();
        }
    }

    fn next(&mut self) {
        match self.state {
            NodeGoNext::DownLeft => {
                self.level += 1;
                self.previous_values.push(self.value);
                self.previous_states.push(self.state);
                self.value = self.calc_value_left().unwrap();
                if self.level == self.max_level {
                    self.sum_max_reached += 1;
                    self.state = NodeGoNext::Up;
                }
            }
            NodeGoNext::DownRight => {
                self.level += 1;
                self.previous_values.push(self.value);
                self.previous_states.push(self.state);
                if let Some(value) = self.calc_value_right() {
                    self.value = value;
                    if self.level == self.max_level {
                        self.sum_max_reached += 1;
                        self.state = NodeGoNext::Up;
                    } else {
                        self.state = NodeGoNext::DownLeft;
                    }
                } else {
                    self.state = NodeGoNext::Up;
                }
            }
            NodeGoNext::Up => {
                if self.level == 0 {
                    self.state = NodeGoNext::Done;
                } else {
                    self.level -= 1;
                    self.value = self.previous_values.pop().unwrap();
                    self.state = self.previous_states.pop().unwrap().next();
                }
            }
            NodeGoNext::Done => {
                println!("No more stepping to do, you are done :)");
            }
        }
    }

    fn calc_value_left(&self) -> Option<u64> {
        if self.value == 0 {
            Some(1)
        } else {
            let len = self.num_digits();
            if len % 2 == 0 {
                Some(self.value_left_part(len / 2))
            } else {
                Some(self.value * 2024)
            }
        }
    }

    fn calc_value_right(&self) -> Option<u64> {
        if self.value == 0 {
            None
        } else {
            let len = self.num_digits();
            if len % 2 == 0 {
                Some(self.value_right_part(len / 2))
            } else {
                None
            }
        }
    }

    fn num_digits(&self) -> u32 {
        self.value.ilog10() + 1
    }

    fn value_left_part(&self, len: u32) -> u64 {
        self.value / 10u64.pow(len)
    }

    fn value_right_part(&self, len: u32) -> u64 {
        self.value % 10u64.pow(len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_num_digits() {
        let node = Tree::new(1, 10);
        assert_eq!(node.num_digits(), 1);
        let node = Tree::new(10, 10);
        assert_eq!(node.num_digits(), 2);
        let node = Tree::new(100, 1);
        assert_eq!(node.num_digits(), 3);
    }

    #[test]
    fn test_node_value_left_part() {
        let node = Tree::new(1234, 10);
        assert_eq!(node.value_left_part(2), 12);
    }

    #[test]
    fn test_node_value_right_part() {
        let node = Tree::new(1234, 10);
        assert_eq!(node.value_right_part(2), 34);
    }

    #[test]
    fn test_node_movements() {
        let mut node = Tree::new(125, 10);
        node.next();
        assert_eq!(node.state, NodeGoNext::DownLeft);
        assert_eq!(node.level, 1);
        assert_eq!(node.value, 253000);
        node.next();
        assert_eq!(node.value, 253);
    }

    #[test]
    fn test_data() {
        let mut total_sum = 0;
        let max_level = 25;
        let values = vec![125, 17];

        for val in values {
            let mut tree = Tree::new(val, max_level);
            tree.walk_through_tree();
            total_sum += tree.sum_max_reached;
        }

        assert_eq!(total_sum, 55312);
    }
}
