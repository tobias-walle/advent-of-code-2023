use anyhow::{Context, Result};
use fancy_regex::{Captures, Regex};
use utils::read_input_file_as_string;

fn main() -> Result<()> {
    let input = read_input_file_as_string().context("Cannot read input")?;

    let result = solve_problem(&input).context("Failed to solve problem")?;
    println!("{result}");
    Ok(())
}

fn solve_problem(input: &str) -> Result<u32> {
    let mut result: u32 = 0;
    let input = replace_digit_words(input);
    println!("{input}");
    for line in input.trim().split('\n') {
        let mut numbers = Vec::<u32>::new();
        for char in line.chars() {
            if let Some(digit) = char.to_digit(10) {
                numbers.push(digit);
            }
        }
        let first = numbers.first().context("No first number")?;
        let last = numbers.last().context("No last number")?;
        result += first * 10 + last;
    }
    Ok(result)
}

fn replace_digit_words(input: &str) -> String {
    let re =
        Regex::new("(?=(one|two|three|four|five|six|seven|eight|nine))").expect("Invalid regex");
    re.replace_all(input, |caps: &Captures| {
        let word = &caps[1];
        dbg!(word);
        let n = match word {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            value => panic!("Missing case '{value}'"),
        };
        n.to_string()
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_example() {
        let input = read_to_string("./example2.txt").unwrap();

        let result = solve_problem(&input).unwrap();
        assert_eq!(result, 281);
    }

    #[test]
    fn test_overlap() {
        let result = replace_digit_words("sevenine");
        dbg!(&result);
        assert!(result.contains('7'));
        assert!(result.contains('9'));
    }
}
