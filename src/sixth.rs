use std::collections::{HashMap, HashSet};
use crate::first::read_file;
use regex::Regex;

pub fn main() {
    let data = read_file("thirdpuzzle.txt").unwrap();
    let sum = calculate_gear_ratios(&data);
    println!("Sum of gear ratios: {}", sum);
}

fn calculate_gear_ratios(lines: &[String]) -> i32 {
    let mut adj: HashMap<(isize, isize), HashSet<i32>> = HashMap::new();
    let re = Regex::new(r"\d+").unwrap();

    for (i, line) in lines.iter().enumerate() {
        for mat in re.find_iter(line) {
            let num = mat.as_str().parse::<i32>().unwrap();
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

            for &(a, b) in &idxs {
                if a >= 0 && a < lines.len() as isize && b >= 0 && b < line.len() as isize &&
                    lines[a as usize].as_bytes()[b as usize] == b'*' {
                    adj.entry((a, b)).or_default().insert(num);
                }
            }
        }
    }

    adj.values()
        .filter(|numbers| numbers.len() == 2)
        .map(|numbers| numbers.iter().copied().product::<i32>())
        .sum()
}




