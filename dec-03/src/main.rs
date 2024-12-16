fn main() {
    let s = std::fs::read_to_string("input").unwrap();

    let resq1 = process_string(&s);
    println!("Result Q1: {}", resq1);

    let resq2 = filter_and_process(&s);
    println!("Result Q2: {}", resq2);
}

fn process_string(s: &str) -> u64 {
    let mut result = 0;
    let mut next_start = 0;

    loop {
        let Some(start) = s[next_start..].find("mul(") else {
            break;
        };
        let start = start + next_start;

        let Some(end) = s[start..].find(")") else {
            break;
        };
        let end = end + start;

        let in_between: Vec<&str> = s[start + 4..end].split(",").collect();
        if in_between.len() != 2 {
            next_start = start + 1;
            continue;
        } else {
            let Ok(a) = in_between[0].parse::<u64>() else {
                next_start = start + 1;
                continue;
            };
            let Ok(b) = in_between[1].parse::<u64>() else {
                next_start = start + 1;
                continue;
            };
            result += a * b;
            next_start = end + 1;
        }
    }

    result
}

fn filter_and_process(s: &str) -> u64 {
    let mut result = 0;

    let mut next_start = 0;

    loop {
        let Some(end) = s[next_start..].find("don't()") else {
            result += process_string(&s[next_start..]);
            break;
        };
        let end = end + next_start;

        result += process_string(&s[next_start..end]);

        let Some(ns) = s[end..].find("do()") else {
            break;
        };
        next_start = ns + end;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_q1() {
        let s = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(process_string(s), 161);
    }

    #[test]
    fn test_example_q2() {
        let s = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(filter_and_process(s), 48);
    }
}
