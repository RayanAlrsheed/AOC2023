use super::solutions::Solvable;
use std::fs;
pub struct Solution();

pub const SOL01: Solution = Solution();

impl Solvable for Solution {
    fn first_solution(&self) -> String {
        let lines = include_str!("inputs/I01.txt");

        let res = lines
            .split("\n")
            .map(|line| get_number01(line))
            .sum::<u32>();
        return res.to_string();
    }

    fn second_solution(&self) -> String {
        let lines = include_str!("inputs/I01.txt");

        let res = lines
            .split("\n")
            .map(|line| get_number02(line))
            .sum::<u32>();
        return res.to_string();
    }
}

fn get_number01(line: &str) -> u32 {
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;

    for c in line.chars() {
        if c.is_numeric() {
            if first.is_none() {
                first = Some(c)
            }

            last = Some(c);
        }
    }

    return (first.unwrap().to_digit(10).unwrap()) * 10 + (last.unwrap().to_digit(10).unwrap());
}

fn get_number02(line: &str) -> u32 {
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;

    for (index, mut c) in line.chars().enumerate() {
        if let Ok(number) = get_digit_in_chars(line.chars().collect(), index) {
            c = number;
        }

        if c.is_numeric() {
            if first.is_none() {
                first = Some(c);
            }

            last = Some(c);
        }
    }

    return (first.unwrap().to_digit(10).unwrap()) * 10 + (last.unwrap().to_digit(10).unwrap());
}

const DIGITS_AS_STRING: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_digit_in_chars(chars: Vec<char>, index: usize) -> Result<char, ()> {
    let mut result: Result<char, ()> = Err(());

    for (current_digit, digit) in DIGITS_AS_STRING.iter().enumerate() {
        let mut current_index = index;
        for (digit_index, c) in digit.chars().enumerate() {
            if current_index >= chars.len() || chars[current_index] != c {
                break;
            }

            if digit_index == digit.len() - 1 {
                result = Ok((char::from_digit(current_digit as u32, 10).expect("msg")));
                break;
            }
            current_index += 1;
        }

        if result.is_ok() {
            break;
        }
    }
    return result;
}
