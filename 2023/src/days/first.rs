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
        let numbers: Vec<&str> = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

        for line in input.lines() {
            let mut numbers_found: Vec<usize> = Vec::new();
            let mut number_counter: Vec<usize> = vec![0; 9];

            for character in line.chars() {
                if character.is_numeric() {
                    numbers_found.push((character as usize) - ('0' as usize));
                    number_counter = vec![0; 9];
                } else {
                    for (index, &verbose_number) in numbers.iter().enumerate() {
                        if verbose_number.chars().nth(number_counter[index]) == Some(character) {
                            // verbose number continuation
                            number_counter[index] += 1;

                            if number_counter[index] == numbers.get(index).unwrap().len() {
                                numbers_found.push(index + 1);
                                number_counter = vec![0; 9];
                                break;
                            }
                        } else if number_counter[index] > 0 {
                            if verbose_number.chars().nth(0) == Some(character) {
                                // restart a count in case a letter is repeating
                                number_counter[index] = 1;
                            } else {
                                // reset old count
                                number_counter[index] = 0;
                            }
                        }
                    }
                }
            }

            if numbers_found.len() == 1 {
                if let Some(first_value) = numbers_found.get(0).cloned() {
                    numbers_found.insert(1, first_value);
                }
            }

            if numbers_found.len() > 1 {
                let number = format!("{}{}", numbers_found.first().unwrap(), numbers_found.last().unwrap());
                values.push(number.parse().unwrap());
            }

            // println!("{:?}", values);
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
    fn test_run_when_scenario_contains_non_verbose_numbers() {
        let mock_input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let output = days::First::run(mock_input.to_string());

        // In this example, the calibration values of these four lines are 12, 38, 15, and 77.
        // Adding these together produces 142.
        assert_eq!(output, "142");
    }

    #[test]
    fn test_run_when_scenario_contains_verbose_numbers_1() {
        let mock_input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        let output = days::First::run(mock_input.to_string());

        // In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76.
        assert_eq!(output, "281");
    }

    #[test]
    fn test_run_when_scenario_contains_verbose_numbers_2() {
        let mock_input = "threerznlrhtkjp23mtflmbrzq395three]\n9sevenvlttm]\n3twochzbv]\nmdxdlh5six5nqfld9bqzxdqxfour]\n422268]\nvdctljvnj2jpgdfnbpfjv1]\ntshl7foureightvzvzdcgt]\n1fourrj";
        let output = days::First::run(mock_input.to_string());

        // In this example, the calibration values are 33, 97, 32, 54, 48, 21, 78, 14
        assert_eq!(output, "377");
    }

    #[test]
    fn test_run_when_scenario_contains_verbose_numbers_3() {
        let mock_input = "47seven811tzhqrrshdm\nvgoneightnsr3fivethreetwornvbz\ngc2\n3sevenxvmzbpknnqninetwofourtwosbpmqk\n4seven8\n2fivegk47gsqtvdms\n1jqgxbmgs4zxkrtvvtsjf1nfsdgtqrmthreeeight\nvbvjdlnfiveninefive162nine";
        let output = days::First::run(mock_input.to_string());

        // In this example, the calibration values are 41, 12, 22, 32, 48, 27, 18, 59
        assert_eq!(output, "259");
    }

    #[test]
    fn test_run_when_scenario_contains_verbose_numbers_4() {
        let mock_input = "ontthreeight";
        let output = days::First::run(mock_input.to_string());

        assert_eq!(output, "33");
    }
}