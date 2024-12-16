enum LineValidity {
    Valid,
    Invalid((usize, usize)), // index of invalid number, index where it should go to be valid
}

fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    let valid_p1 = part1(&s, false);
    let res1 = center_sum(valid_p1);
    println!("Part 1: {}", res1);

    let res2 = part2(&s);
    println!("Part 2: {}", res2);
}

fn center_sum(valid_lines: Vec<Vec<u64>>) -> u64 {
    let mut center_sum = 0;
    for line in valid_lines {
        let ind = line.len().div_ceil(2) - 1;
        center_sum += line[ind];
    }
    center_sum
}

fn parse_input(s: &str) -> (Vec<(u64, u64)>, Vec<Vec<u64>>) {
    let mut lines = s.lines();
    let mut rules = vec![];
    let mut lists: Vec<Vec<u64>> = vec![];

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let mut parts = line.split('|');
        let a: u64 = parts.next().unwrap().parse().unwrap();
        let b: u64 = parts.next().unwrap().parse().unwrap();
        rules.push((a, b));
    }

    for line in lines {
        let list = line.split(',').map(|x| x.parse::<u64>().unwrap()).collect();
        lists.push(list);
    }

    (rules, lists)
}

fn check_line_validity(line: &[u64], rules: &Vec<(u64, u64)>) -> LineValidity {
    for it in 1..line.len() {
        let left = line[it - 1];
        let right = line[it];
        for rule in rules {
            if rule.1 == left && line[it..].iter().any(|&x| x == rule.0) {
                // find the index of the invalid number
                let ind_inv = line[it..].iter().position(|&x| x == rule.0).unwrap()+it;
                return LineValidity::Invalid((ind_inv, it - 1));
            } else if rule.0 == right && line[..it].iter().any(|&x| x == rule.1) {
                let ind_inv = line[..it].iter().position(|&x| x == rule.1).unwrap()+it;
                return LineValidity::Invalid((ind_inv, it));
            }
        }
    }
    LineValidity::Valid
}

/// Part 1, returns the valid lines.
fn part1(s: &str, get_invalid_lines: bool) -> Vec<Vec<u64>> {
    let (rules, lines) = parse_input(s);
    let mut valid_lines = vec![];
    let mut invalid_lines = vec![];
    for line in &lines {
        match check_line_validity(line, &rules) {
            LineValidity::Valid => {
                valid_lines.push(line.to_vec());
            }
            _ => {
                invalid_lines.push(line.to_vec());
            }
        }

    }
    if !get_invalid_lines {
        valid_lines
    } else {
        invalid_lines
    }
}

fn part2(s: &str) -> u64 {
    let (rules, _) = parse_input(s);
    let mut good_lines = vec![];
    let invalid_lines = part1(s, true);

    for mut line in invalid_lines {
        loop {
            match check_line_validity(&line, &rules) {
                LineValidity::Valid => {
                    good_lines.push(line);
                    break;
                }
                LineValidity::Invalid((ind_inv, ind)) => {
                    let tmp = line.remove(ind_inv);
                    line.insert(ind, tmp);
                }
            }

        }
    }

    center_sum(good_lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TDAT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_read_and_split() {
        let (rules, lists) = parse_input(TDAT);
        assert_eq!(rules[3], (97, 47));
        assert_eq!(lists[5], vec![97, 13, 75, 29, 47]);
    }

    #[test]
    fn test_part1() {
        let valid_lines = part1(TDAT, false);
        assert_eq!(center_sum(valid_lines), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TDAT), 123);
    }
}
