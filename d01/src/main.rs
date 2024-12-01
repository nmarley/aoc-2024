use std::collections::HashMap;
use std::fs;

fn main() {
    let input1 = fs::read_to_string("d01/input").unwrap();

    let solution1 = inner_solve(&input1);
    println!("Solution 1: {}", solution1);

    let solution2 = inner_solve2(&input1);
    println!("Solution 2: {}", solution2);
}

pub fn gen_vectors<S: AsRef<str>>(s: S) -> (Vec<u32>, Vec<u32>) {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    for line in s.as_ref().lines() {
        let mut nums = line.split("   ").map(|n| n.parse::<u32>().unwrap());
        let a = nums.next().unwrap();
        let b = nums.next().unwrap();
        left.push(a);
        right.push(b);
    }

    (left, right)
}

pub fn inner_solve<S: AsRef<str>>(s: S) -> u32 {
    let (mut left, mut right) = gen_vectors(s.as_ref());

    left.sort_unstable();
    right.sort_unstable();

    let mut sum_differences: u32 = 0;
    for (a, b) in left.iter().zip(right.iter()) {
        let diff = if a > b { a - b } else { b - a };
        sum_differences += diff;
    }

    sum_differences
}

pub fn inner_solve2<S: AsRef<str>>(s: S) -> u32 {
    let (left, right) = gen_vectors(s.as_ref());

    let mut right_freq: HashMap<u32, u32> = HashMap::new();
    for r in right.iter() {
        let count = right_freq.entry(*r).or_insert(0);
        *count += 1;
    }

    let mut total: u32 = 0;
    for l in left.iter() {
        let sim = right_freq.get(l).unwrap_or(&0);
        total += *sim * l;
    }

    total
}

#[cfg(test)]
mod test {
    use super::{inner_solve, inner_solve2};

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn gen_vectors() {
        let result = super::gen_vectors(INPUT);
        assert_eq!((vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]), result);
    }

    #[test]
    fn result1() {
        let result = inner_solve(INPUT);
        assert_eq!(11, result);
    }

    #[test]
    fn result2() {
        let result = inner_solve2(INPUT);
        assert_eq!(31, result);
    }
}
