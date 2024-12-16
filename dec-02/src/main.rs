use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = "input";
    let lines = read_lines(filename).unwrap();

    // process the reports
    let mut overall_vec = vec![];
    for line in lines.map_while(Result::ok) {
        let entries: Vec<isize> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        overall_vec.push(entries);
    }

    let (total_safe, total_safe_dampened) = check_input(overall_vec);
    println!("Total safe reports: {}", total_safe);
    println!("Total safe reports dampened: {}", total_safe_dampened);
}

fn check_input(input: Vec<Vec<isize>>) -> (usize, usize) {
    let mut total_safe = 0;
    let mut total_safe_damp = 0;

    for inp in input {
        if process_report(&inp, false) {
            total_safe += 1;
        }

        if process_report(&inp, true) {
            total_safe_damp += 1;
        }
    }
    (total_safe, total_safe_damp)
}

/// Process each individual report and return true / false if it is safe / unsafe
fn process_report(entries: &[isize], dampened: bool) -> bool {
    let mut parts = entries.to_vec();
    let mut diff = parts[1..]
        .iter()
        .zip(&parts[..parts.len() - 1])
        .map(|(a, b)| a - b)
        .collect::<Vec<isize>>();

    if !majority_positive(&diff) {
        parts = parts.iter().map(|x| x * -1).collect();
        diff = diff.iter().map(|x| x * -1).collect();
    }

    if diff.iter().filter(|&x| !(1..=3).contains(x)).count() == 0 {
        return true;
    }

    if !dampened {
        return false;
    }

    // Brute force dampening - whatever
    for rit in 0..parts.len() {
        let mut parts2 = parts.clone();
        parts2.remove(rit);
        if process_report(&parts2, false) {
            return true;
        }
    }
    false
}

/// Find out if majority of entries in a slice are positive
fn majority_positive(slice: &[isize]) -> bool {
    slice.iter().filter(|&x| x > &0).count() >= slice.iter().filter(|&x| x < &0).count()
}

/// Read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_example() {
        let input = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        let (total_safe, total_safe_dampened) = check_input(input);
        assert_eq!(total_safe, 2);
        assert_eq!(total_safe_dampened, 4);
    }

    #[test]
    fn check_dampened() {
        let input = vec![
            vec![8, 7, 10, 12, 13],
            vec![7, 7, 10, 12, 13],
            vec![7, 10],
            vec![2, 1, 5, 6],
            vec![50, 51, 47, 46],
            vec![1, 2, 15, 4],
            vec![48, 46, 47, 49, 51, 54, 56],
            vec![1, 1, 2, 3, 4, 5],
            vec![1, 2, 3, 4, 5, 5],
            vec![5, 1, 2, 3, 4, 5],
            vec![1, 4, 3, 2, 1],
            vec![1, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 3],
            vec![9, 8, 7, 6, 7],
        ];
        let (_, total_safe_dampened) = check_input(input.clone());
        assert_eq!(total_safe_dampened, input.len());
    }
}
