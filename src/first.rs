use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn find_calibration_value(s: &str) -> Option<u32> {
    let mut first_digit = None;
    let mut last_digit = None;

    for c in s.chars() {
        if c.is_digit(10) {
            if first_digit.is_none() {
                first_digit = Some(c);
            }
            last_digit = Some(c);
        }
    }

    match (first_digit, last_digit) {
        (Some(f), Some(l)) => Some(f.to_digit(10).unwrap() * 10 + l.to_digit(10).unwrap()),
        _ => None,
    }
}

pub fn read_file(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    reader.lines().collect()
}

pub(crate) fn main() {
    let string_vector = read_file("puzzleinput.txt").unwrap();
    let mut sum = 0;

    for s in string_vector {
        let calibration_value = find_calibration_value(&s);
        match calibration_value {
            Some(n) => sum += n,
            None => println!("No calibration value found for {}", s),
        }
    }
    println!("The sum of all calibration values is {}", sum);

}