use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // read the input file into two vectors
    let mut col1: Vec<u64> = vec![];
    let mut col2: Vec<u64> = vec![];
    
    let fname = "./src/input1";
    let f = File::open(fname).expect("file not found");
    let lines = io::BufReader::new(f).lines();

    for line in lines {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let a: u64 = parts.next().unwrap().parse().unwrap();
        let b: u64 = parts.next().unwrap().parse().unwrap();
        col1.push(a);
        col2.push(b);
    }

    // Now sor the columns
    col1.sort();
    col2.sort();

    // Subract the two columns
    let mut result = 0;
    for it in 0..col1.len() {
        result += col1[it].abs_diff(col2[it]);
    }

    println!("Distance: {}", result);

    // Part 2 - calculate the similarity score
    let mut result = 0;
    for c1 in col1 {
        let occ_c2 = col2.iter().filter(|&x| *x == c1).count();
        result += c1 * occ_c2 as u64;
    }

    println!("Similarity score: {}", result);
}
