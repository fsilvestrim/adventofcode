mod days;

use std::io;
use days::challenge_trait::Challenge;
use crate::days::Second;

fn main() {
    println!("{}", Second::challenge());
    println!("Type the challenge input please: ");

    let mut input_lines: Vec<String> = Vec::new();

    loop {
        let mut input_line = String::new();

        match io::stdin().read_line(&mut input_line) {
            Ok(bytes_read) => {
                let line = input_line.trim().to_string();
                if bytes_read == 1 && line.is_empty() {
                    break;
                }
                input_lines.push(line);
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }
    }

    println!("Result: {}", Second::run(input_lines.join("\n")));
}
