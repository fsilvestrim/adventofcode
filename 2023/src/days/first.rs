use super::challenge_trait::Challenge;

pub struct First;

impl Challenge for First {
    fn challenge() -> String {
        "See https://adventofcode.com/2023/day/1".to_string()
    }

    /*
    On each line, the calibration value can be found by combining the first digit and the last digit
    (in that order) to form a single two-digit number.
     */
    fn run(input: String) -> String {
        let mut values: Vec<u32> = Vec::new();

        for line in input.lines() {
            let mut first_number : Option<char> = None;
            let mut last_number : Option<char> = None;
            for character in line.chars() {
                if character.is_numeric() {
                    if first_number.is_none() {
                        first_number = Some(character);
                    }
                    last_number = Some(character);
                }
            }
            if first_number.is_some() && last_number.is_some() {
                let mut number = String::new();
                number.push(first_number.unwrap());
                number.push(last_number.unwrap());

                let result: Result<u32, _> = number.parse();

                // Handle the result
                match result {
                    Ok(number) => {
                        values.push(number);
                    }
                    Err(error) => {
                        eprintln!("Failed to parse string as number: {}", error);
                    }
                }
            }
        }

        values.iter().sum::<u32>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::days;
    use crate::days::challenge_trait::Challenge;

    #[test]
    fn test_print_challenge() {
        let output = days::First::challenge();
        assert!(output.contains("https://adventofcode.com/2023/day/1"));
    }

    #[test]
    fn test_run() {
        let mock_input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";

        // In this example, the calibration values of these four lines are 12, 38, 15, and 77.
        // Adding these together produces 142.

        let output = days::First::run(mock_input.to_string());

        // In this example, the calibration values of these four lines are 12, 38, 15, and 77.
        // Adding these together produces 142.
        assert_eq!(output, "142");
    }
}