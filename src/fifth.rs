use std::collections::HashMap;
use crate::first::read_file;
use regex::Regex;

pub fn main() {
    let data = read_file("thirdpuzzle.txt").unwrap();
    let sum = sum_surrounded_numbers(&data);
    println!("Sum of surrounded numbers: {}", sum);
    }


fn sum_surrounded_numbers(lines: &[String]) -> i32
{
    let mut ans = 0;
    let re = Regex::new(r"\d+").unwrap();
    for (i, line) in lines.iter().enumerate() {
        for mat in re.find_iter(line) {
            let start = mat.start() as isize;
            let end = mat.end() as isize;
            let i = i as isize;

            let above_below = (start..=end).flat_map(|x| [(i - 1, x), (i + 1, x)]);
            let diagonals_sides = [
                (i - 1, start - 1), (i - 1, end),
                (i + 1, start - 1), (i + 1, end),
                (i, start - 1), (i, end)
            ];

            let idxs: Vec<(isize, isize)> = above_below.chain(diagonals_sides.iter().copied()).collect();
            let count = idxs.iter().filter(|&&(a, b)| {
                a >= 0 && a < lines.len() as isize &&
                    b >= 0 && b < lines[a as usize].len() as isize &&
                    lines[a as usize].as_bytes()[b as usize] != b'.'

            }).count();

            if count > 0 {
                let number = mat.as_str().parse::<i32>().unwrap_or(0);
                ans += number;
            }
        }
    }

    ans
}
