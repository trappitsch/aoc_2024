use std::io::{self, BufRead};

const PAT1: &str = "XMAS";
const PAT2: &str = "SAMX";

const PAT3: &str = "MAS";
const PAT4: &str = "SAM";

fn main() {
    let s = std::fs::File::open("input").unwrap();
    let s = io::BufReader::new(s).lines();
    let data = s.map(|x| x.unwrap()).collect::<Vec<String>>();

    let res1 = part1(&data);

    println!("Part 1: {}", res1);

    let res2 = part2(&data);
    println!("Part 2: {}", res2);
}

fn part1(s: &Vec<String>) -> u64 {
    let mut data: Vec<Vec<String>> = vec![];
    for i in s {
        let mut row: Vec<String> = vec![];
        for j in i.chars() {
            row.push(j.into());
        }
        data.push(row);
    }

    let mut cnt = 0;

    // search each line forward, backward
    for line in &data {
        for it in 0..line.len() - 3 {
            let cmp = line[it..it + 4].join("");
            if cmp == PAT1 || cmp == PAT2 {
                cnt += 1;
            }
        }
    }

    // search top to bottom
    for chit in 0..data[0].len() {
        for lit in 0..data.len() - 3 {
            let mut cmp = ["", "", "", ""];
            for jt in 0..4 {
                cmp[jt] = &data[jt + lit][chit]
            }
            let cmp = cmp.join("");
            if cmp == PAT1 || cmp == PAT2 {
                cnt += 1;
            }
        }
    }

    // search diagonals top left to bottom right
    for chit in 0..data[0].len() - 3 {
        for lit in 0..data.len() - 3 {
            let mut cmp = ["", "", "", ""];
            for jt in 0..4 {
                cmp[jt] = &data[jt + lit][chit + jt]
            }
            let cmp = cmp.join("");
            if cmp == PAT1 || cmp == PAT2 {
                cnt += 1;
            }
        }
    }

    // search diagonals top right to bottom left
    for chit in 3..data[0].len() {
        for lit in 0..data.len() - 3 {
            let mut cmp = ["", "", "", ""];
            for jt in 0..4 {
                cmp[jt] = &data[jt + lit][chit - jt]
            }
            let cmp = cmp.join("");
            if cmp == PAT1 || cmp == PAT2 {
                cnt += 1;
            }
        }
    }
    cnt
}

fn part2(s: &Vec<String>) -> u64 {
    let mut data: Vec<Vec<String>> = vec![];
    for i in s {
        let mut row: Vec<String> = vec![];
        for j in i.chars() {
            row.push(j.into());
        }
        data.push(row);
    }

    let mut cnt = 0;

    for lit in 1..data.len() - 1 {
        for chit in 1..data[lit].len() - 1 {
            let mut cmp1 = ["", "", ""];
            let mut cmp2 = ["", "", ""];
            for it in 0..=2 {
                cmp1[it] = &data[lit - 1 + it][chit + 1 - it];
                cmp2[it] = &data[lit - 1 + it][chit - 1 + it];
            }
            let cmp1 = cmp1.join("");
            let cmp2 = cmp2.join("");
            if (cmp1 == PAT3 || cmp1 == PAT4) && (cmp2 == PAT3 || cmp2 == PAT4) {
                println!("cmp1 {}, cmp2 {}, lit {}, chit {}", cmp1, cmp2, lit, chit);
                cnt += 1;
            }
        }
    }

    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let s: Vec<String> = vec![
            "MMMSXXMASM".into(),
            "MSAMXMSMSA".into(),
            "AMXSXMAAMM".into(),
            "MSAMASMSMX".into(),
            "XMASAMXAMM".into(),
            "XXAMMXXAMA".into(),
            "SMSMSASXSS".into(),
            "SAXAMASAAA".into(),
            "MAMMMXMMMM".into(),
            "MXMXAXMASX".into(),
        ];
        assert_eq!(part1(&s), 18);
    }

    #[test]
    fn test_part1_edge1() {
        let s: Vec<String> = vec![
            "XMASAMX".into(),
            "MM000MM".into(),
            "A0A0A0A".into(),
            "S00S00S".into(),
            "A0A0A0A".into(),
            "MM000MM".into(),
            "XMASAMX".into(),
        ];
        assert_eq!(part1(&s), 12);
    }

    #[test]
    fn test_part2() {
        let s: Vec<String> = vec![
            "MMMSXXMASM".into(),
            "MSAMXMSMSA".into(),
            "AMXSXMAAMM".into(),
            "MSAMASMSMX".into(),
            "XMASAMXAMM".into(),
            "XXAMMXXAMA".into(),
            "SMSMSASXSS".into(),
            "SAXAMASAAA".into(),
            "MAMMMXMMMM".into(),
            "MXMXAXMASX".into(),
        ];
        assert_eq!(part2(&s), 9);
    }
}
