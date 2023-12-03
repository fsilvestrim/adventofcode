use crate::challenge::challenge_trait::Challenge;

pub struct Third;

impl Challenge for Third {
    fn challenge() -> String {
        "See https://adventofcode.com/2023/day/3".to_string()
    }

    /*
     */
    fn run(input: String) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::challenge::challenge_trait::Challenge;
    use crate::days;

    #[test]
    fn test_print_challenge() {
        let output = days::Third::challenge();
        assert!(output.contains("https://adventofcode.com/2023/day/3"));
    }

    #[test]
    fn test_run_when_scenario_contains_non_verbose_numbers() {
        let mock_input = "";
        let output = days::Third::run(mock_input.to_string());

        assert_eq!(output, "");
    }
}