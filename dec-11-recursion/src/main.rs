//! Well, a bit of a cheat, but had to look up how to do this recursion. Let's try...
use std::collections::HashMap;

/// Return the number of stones in this recursion.
fn recursive_walk(stone: u64, level: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    // We are doing no levels at all...
    if level == 0 {
        return 1;
    }

    if let Some(res) = cache.get(&(stone, level)) {
        return *res;
    }

    let res = match stone {
        0 => recursive_walk(1, level - 1, cache),
        n => {
            let num_digits = n.ilog10() + 1;
            if num_digits % 2 == 0 {
                let left = n / 10u64.pow(num_digits / 2);
                let right= n % 10u64.pow(num_digits / 2);
                recursive_walk(left, level - 1, cache) + recursive_walk(right, level - 1, cache)
            } else {
                recursive_walk(n * 2024, level - 1, cache)
            }
        }
    };

    cache.insert((stone, level), res);
    res
}

fn main() {
    let mut cache: HashMap<(u64, u64), u64> = HashMap::new();

    let level = 75;

    let data = std::fs::read_to_string("input").unwrap();
    let data: Vec<&str> = data.split_whitespace().collect();
    let data: Vec<u64> = data.iter().map(|x| x.parse::<u64>().unwrap()).collect();

    let sum_p1: u64 = data.iter().map(|v| recursive_walk(*v, level, &mut cache)).sum();
    println!("At level {}: {}", level, sum_p1);
}
