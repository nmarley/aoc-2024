use std::collections::HashMap;
use std::fs;

fn main() {
    let input1 = fs::read_to_string("d02/input").unwrap();

    let solution1 = inner_solve(&input1);
    println!("Solution 1: {}", solution1);

    let solution2 = inner_solve2(&input1);
    println!("Solution 2: {}", solution2);
}

pub fn gen_reports<S: AsRef<str>>(s: S) -> Vec<Vec<u32>> {
    let mut reports: Vec<Vec<u32>> = vec![];

    for line in s.as_ref().lines() {
        let nums = line
            .split(' ')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        reports.push(nums);
    }

    reports
}

pub fn inner_solve<S: AsRef<str>>(s: S) -> u32 {
    let reports = gen_reports(s.as_ref());
    reports.iter().filter(|r| is_safe(r)).count() as u32
}

// Must be:
// 1. Either strictly increasing or strictly decreasing
// 2. Each adjacent pair must have a difference of at least 1, at most 3
fn is_safe(r: &[u32]) -> bool {
    let mut prev = r[0];
    let (mut increasing, mut decreasing) = (true, true);
    for curr in r.iter().skip(1) {
        let diff = if curr > &prev {
            curr - prev
        } else {
            prev - curr
        };
        // short-circuit if non-gradual increase/decrease
        if diff < 1 || diff > 3 {
            return false;
        }

        if *curr > prev {
            decreasing = false;
        } else if *curr < prev {
            increasing = false;
        }

        if !increasing && !decreasing {
            return false;
        }
        prev = *curr;
    }
    true
}

pub fn inner_solve2<S: AsRef<str>>(s: S) -> u32 {
    let reports = gen_reports(s.as_ref());

    // let mut right_freq: HashMap<u32, u32> = HashMap::new();
    // for r in right.iter() {
    //     let count = right_freq.entry(*r).or_insert(0);
    //     *count += 1;
    // }
    //
    // let mut total: u32 = 0;
    // for l in left.iter() {
    //     let sim = right_freq.get(l).unwrap_or(&0);
    //     total += *sim * l;
    // }

    0
}

fn is_increasing(v: &Vec<u32>) -> bool {
    let mut prev = v[0];
    for n in v.iter().skip(1) {
        if *n < prev {
            return false;
        }
        prev = *n;
    }
    true
}

fn is_decreasing(v: &Vec<u32>) -> bool {
    let mut prev = v[0];
    for n in v.iter().skip(1) {
        if *n > prev {
            return false;
        }
        prev = *n;
    }
    true
}

#[cfg(test)]
mod test {
    use super::{inner_solve, inner_solve2};

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn gen_reports() {
        let result = super::gen_reports(INPUT);
        assert_eq!(
            vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9],
            ],
            result
        );
    }

    #[test]
    fn result1() {
        let result = inner_solve(INPUT);
        assert_eq!(2, result);
    }

    #[test]
    fn result2() {
        let result = inner_solve2(INPUT);
        assert_eq!(31, result);
    }
}
