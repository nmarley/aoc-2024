use once_cell::sync::Lazy;
use regex::Regex;
use std::fs;

static RE_MULS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r#"(?x)
        (?:mul\((\d+),(\d+)\))
        | (?:do\(\))
        | (?:don't\(\))
    "#,
    )
    .unwrap()
});

fn main() {
    let input = fs::read_to_string("d03/input").unwrap();

    let solution1 = inner_solve(&input);
    println!("Solution 1: {}", solution1);

    let solution2 = inner_solve2(&input);
    println!("Solution 2: {}", solution2);
}

pub fn scan_muls<S: AsRef<str>>(s: S, only_enabled: bool) -> Vec<(u32, u32)> {
    let mut tuples: Vec<(u32, u32)> = vec![];

    let mut enabled = true;
    for cap in RE_MULS.captures_iter(s.as_ref()) {
        match cap[0].as_ref() {
            "do()" => {
                enabled = true;
            }
            "don't()" => {
                enabled = false;
            }
            _ => {
                let a = cap[1].parse::<u32>().unwrap();
                let b = cap[2].parse::<u32>().unwrap();
                if !only_enabled || enabled {
                    tuples.push((a, b));
                }
            }
        };
    }

    tuples
}

pub fn inner_solve<S: AsRef<str>>(s: S) -> u32 {
    let tuples = scan_muls(s.as_ref(), false);
    tuples.iter().map(|(a, b)| a * b).sum::<u32>()
}

pub fn inner_solve2<S: AsRef<str>>(s: S) -> u32 {
    let tuples = scan_muls(s.as_ref(), true);
    tuples.iter().map(|(a, b)| a * b).sum::<u32>()
}

#[cfg(test)]
mod test {
    use super::{inner_solve, inner_solve2};

    const INPUT1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn scan_muls_all() {
        let result = super::scan_muls(INPUT1, false);
        assert_eq!(vec![(2, 4), (5, 5), (11, 8), (8, 5)], result);
    }

    #[test]
    fn scan_muls_enabled() {
        let result = super::scan_muls(INPUT2, true);
        assert_eq!(vec![(2, 4), (8, 5)], result);
    }

    #[test]
    fn result1() {
        let result = inner_solve(INPUT1);
        assert_eq!(161, result);
    }

    #[test]
    fn result2() {
        let result = inner_solve2(INPUT2);
        assert_eq!(48, result);
    }
}
