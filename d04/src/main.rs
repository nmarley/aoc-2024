use std::fs;

fn main() {
    let input = fs::read_to_string("d04/input").unwrap();

    let solution1 = inner_solve(&input);
    println!("Solution 1: {}", solution1);

    // let solution2 = inner_solve2(&input);
    // println!("Solution 2: {}", solution2);
}

pub fn count_xmas<S: AsRef<str>>(s: S, only_enabled: bool) -> u32 {
    7
}

pub fn inner_solve<S: AsRef<str>>(s: S) -> u32 {
    let tuples = scan_muls(s.as_ref(), false);
    tuples.iter().map(|(a, b)| a * b).sum::<u32>()
}

// pub fn inner_solve2<S: AsRef<str>>(s: S) -> u32 {
//     let tuples = scan_muls(s.as_ref(), true);
//     tuples.iter().map(|(a, b)| a * b).sum::<u32>()
// }

#[cfg(test)]
mod test {
    // use super::{inner_solve, inner_solve2};
    use super::inner_solve;

    const INPUT1: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn result1() {
        let result = inner_solve(INPUT1);
        assert_eq!(18, result);
    }

    // #[test]
    // fn result2() {
    //     let result = inner_solve2(INPUT2);
    //     assert_eq!(48, result);
    // }
}
