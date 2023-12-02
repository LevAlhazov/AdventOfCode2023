use std::collections::HashMap;
use crate::first::read_file;

pub fn main() {
    let games_data = read_file("secondpuzzleinput.txt").unwrap();
    let total_power = games_data.iter().map(|game_data| {
        let sets = game_data.split(": ").nth(1).unwrap().split("; ");
        let mut max_cubes = HashMap::new();

        for set in sets {
            set.split(", ").for_each(|cube| {
                let parts: Vec<_> = cube.split_whitespace().collect();
                let count: i32 = parts[0].parse().unwrap();
                let color = parts[1];
                let entry = max_cubes.entry(color).or_insert(0);
                *entry = i32::max(*entry, count);
            });
        }

        max_cubes.values().product::<i32>()
    }).sum::<i32>();

    println!("Total power of all sets: {}", total_power);
}
