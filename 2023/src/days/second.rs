use super::challenge_trait::Challenge;

pub struct Second;

impl Challenge for Second {
    fn challenge() -> String {
        "See https://adventofcode.com/2023/day/2".to_string()
    }

    /*
    The Elf would first like to know which games would have been possible if the bag
    contained only 12 red cubes, 13 green cubes, and 14 blue cubes?
     */
    fn run(input: String) -> String {
        let cube_colors : Vec<&str> = vec!["red", "green", "blue"];
        let bag_content : Vec<usize> = vec![12, 13, 14];

        let mut possible_games: Vec<u32> = Vec::new();
        let mut powers: Vec<u32> = Vec::new();

        // Example: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        for (index, line) in input.lines().enumerate() {
            let game_idx = index+1;
            let line_without_headline = line.trim_start_matches(format!("Game {}:", game_idx).as_str());
            let game: Vec<&str> = line_without_headline.split(";").collect();

            let mut valid = true;
            let mut min_cube_amounts = vec![0, 0, 0];

            for set in game {
                let cube_color_combination: Vec<&str> = set.trim().split(",").collect();
                for combination in cube_color_combination {
                    let amount_color: Vec<&str> = combination.trim().split(" ").collect();
                    if amount_color.len() > 1 {
                        let amount_str = amount_color.get(0).unwrap();
                        if let Ok(amount) = amount_str.parse::<usize>() {
                            let color = amount_color.get(1).unwrap();
                            if let Some(color_index) = cube_colors.iter().position(|&c| c == *color) {
                                min_cube_amounts[color_index] = min_cube_amounts[color_index].max(amount);
                                let compared_amount= bag_content[color_index];
                                if amount > compared_amount {
                                    valid = false;
                                }
                            }
                        } else {
                            valid = false;
                        }
                    }
                }
            }

            let power = min_cube_amounts.iter().fold(1, |acc, &min| acc * min);
            powers.push(power as u32);

            if valid {
                possible_games.push(game_idx as u32);
            }
        }

        let games_result = possible_games.iter().sum::<u32>().to_string();
        let powers_result = powers.iter().sum::<u32>().to_string();
        format!( "{},{}", games_result, powers_result)
    }
}

#[cfg(test)]
mod tests {
    use crate::days;
    use crate::days::challenge_trait::Challenge;

    #[test]
    fn test_print_challenge() {
        let output = days::Second::challenge();
        assert!(output.contains("https://adventofcode.com/2023/day/2"));
    }

    #[test]
    fn test_run_when_scenario_contains_non_verbose_numbers() {
        let mock_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let output = days::Second::run(mock_input.to_string());

        // In the example above, games 1, 2, and 5 would have been possible if the bag had been
        // loaded with that configuration. However, game 3 would have been impossible because
        // at one point the Elf showed you 20 red cubes at once; similarly,
        // game 4 would also have been impossible because the Elf showed you 15 blue cubes at once.
        // If you add up the IDs of the games that would have been possible, you get 8.

        // The power of the minimum set of cubes in game 1 is 48. In games 2-5 it was 12, 1560, 630,
        // and 36, respectively. Adding up these five powers produces the sum 2286.
        assert_eq!(output, "8,2286");
    }

}