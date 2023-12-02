
use crate::first;
use first::read_file;


fn find_calibration_value(s: &str) -> Option<u32> {
    let digit_map = [("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5),
        ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)];
    let mut first_digit = None;
    let mut last_digit = None;


    for (index, c) in s.char_indices() {
        if c.is_digit(10) {
            let digit = c.to_digit(10).unwrap();
            if first_digit.is_none() {
                first_digit = Some(digit);
            }
            last_digit = Some(digit);
        } else {
            for (word, value) in digit_map.iter() {
                if s[index..].starts_with(word) {
                    if first_digit.is_none() {
                        first_digit = Some(*value);
                    }
                    last_digit = Some(*value);
                    break;
                }
            }
        }
    }

    match (first_digit, last_digit) {
        (Some(f), Some(l)) => Some(f * 10 + l),
        _ => None,
    }
}


pub(crate) fn main() {
    // read the strings from puzzleinput.txt
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
