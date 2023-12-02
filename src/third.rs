use std::collections::HashMap;
use crate::first::read_file;

pub fn main() {
    let input = read_file("secondpuzzleinput.txt").unwrap();
    let max_cubes = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let result = calculate_possible_games(&input, &max_cubes);
    println!("Sum of IDs of possible games: {}", result);
}

fn calculate_possible_games(games_data: &[String], max_cubes: &HashMap<&str, i32>) -> i32 {
    games_data.iter().filter_map(|game_data| {
        let parts: Vec<_> = game_data.split(": ").collect();
        let game_id: i32 = parts[0].split_whitespace().last().unwrap().parse().unwrap();
        let sets = parts[1].split("; ");

        let is_possible = sets.clone().all(|set| {
            set.split(", ").all(|cube| {
                let parts: Vec<_> = cube.split_whitespace().collect();
                let count: i32 = parts[0].parse().unwrap();
                let color = parts[1];
                count <= *max_cubes.get(color).unwrap_or(&0)
            })
        });

        if is_possible { Some(game_id) } else { None }
    }).sum()
}

