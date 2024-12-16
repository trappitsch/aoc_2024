fn main() {
    let data = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

/// Check if any combinations of + and * for input numbers can result in result.
fn check_line_p1(result: u64, input: &[u64]) -> bool {
    let len_ops = input.len() - 1;

    let total_combinations = 2u64.pow(len_ops as u32);

    for ops_bin in 0..total_combinations {
        let mut result_test = input[0];
        for ot in (0..len_ops).rev() {
            match ops_bin >> ot & 1 {
                0 => {
                    result_test += input[len_ops - ot];
                }
                _ => {
                    result_test *= input[len_ops - ot];
                }
            }
        }
        if result_test == result {
            return true;
        }
    }
    false
}

/// Check if any combinations of +, * and concat for input numbers can result in result.
fn check_line_p2(result: u64, input: &[u64]) -> bool {
    let len_ops = input.len() - 1;

    let total_combinations = 3u64.pow(len_ops as u32);

    for ops_bin in 0..total_combinations {
        let mut result_test = input[0];
        for ot in (0..len_ops).rev() {
            match ops_bin / 3u64.pow(ot as u32) % 3 {
                0 => {
                    result_test += input[len_ops - ot];
                }
                1 => {
                    result_test *= input[len_ops - ot];
                }
                _ => {
                    let rhs = input[len_ops - ot];
                    let multiplier = rhs.ilog10() + 1;
                    result_test *= 10u64.pow(multiplier);
                    result_test += rhs;
                }
            }
        }
        if result_test == result {
            return true;
        }
    }
    false
}

fn parse_data(data: &str) -> (Vec<u64>, Vec<Vec<u64>>) {
    let mut results = vec![];
    let mut input = vec![];

    for line in data.lines() {
        let part = line.split(": ").collect::<Vec<&str>>();
        results.push(part[0].parse::<u64>().unwrap());
        input.push(
            part[1]
                .split(" ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect(),
        );
    }
    (results, input)
}

fn part1(data: &str) -> u64 {
    let (results, input) = parse_data(data);
    let mut total = 0;
    for (it, result) in results.iter().enumerate() {
        if check_line_p1(*result, &input[it]) {
            total += result;
        }
    }
    total
}

fn part2(data: &str) -> u64 {
    let (results, input) = parse_data(data);
    let mut total = 0;
    for (it, result) in results.iter().enumerate() {
        if check_line_p2(*result, &input[it]) {
            total += result;
        }
    }
    total
}
#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_parse_data() {
        let (result, input) = parse_data(DATA);
        assert_eq!(
            result,
            vec![190, 3267, 83, 156, 7290, 161011, 192, 21037, 292]
        );
        assert_eq!(input[0], vec![10, 19]);
        assert_eq!(input[1], vec![81, 40, 27]);
    }

    #[test]
    fn test_check_line() {
        let (result, input) = parse_data(DATA);
        let expected_results = [true, true, false, false, false, false, false, false, true];
        for (i, r) in result.iter().enumerate() {
            let result = check_line_p1(*r, &input[i]);
            assert_eq!(result, expected_results[i]);
        }
    }

    #[test]
    fn test_part1_ex() {
        assert_eq!(part1(DATA), 3749);
    }

    #[test]
    fn test_pat2_ex() {
        assert_eq!(part2(DATA), 11387);
    }
}
