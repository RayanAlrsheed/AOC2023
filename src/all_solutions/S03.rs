use super::solutions::Solvable;
use regex::Regex;

pub struct Solution();

#[derive(Debug)]
struct Number {
    value: u32,
    line: i32,
    start: i32,
    end: i32,
    active: bool,
}

//x, y, is_gear
#[derive(Debug)]
struct Symbol(i32, i32, bool);

impl Solvable for Solution {
    fn first_solution(&self) -> String {
        let (mut numbers, symbols) = parse();

        for symbol in &symbols {
            for number in &mut numbers {
                if is_adjacent(&symbol, &number) {
                    number.active = true;
                }
            }
        }

        let result: String = numbers
            .iter()
            .map(|number| if number.active { number.value } else { 0 })
            .sum::<u32>()
            .to_string();

        return result;
    }

    fn second_solution(&self) -> String {
        let (numbers, symbols) = parse();

        let mut result: u32 = 0;

        for symbol in symbols {
            if !symbol.2 {
                continue;
            }

            let mut gear_values: Vec<u32> = vec![];

            for number in &numbers {
                if is_adjacent(&symbol, &number) {
                    gear_values.push(number.value);
                }
            }

            if gear_values.len() == 2 {
                result += gear_values.iter().product::<u32>();
            }
        }

        result.to_string()
    }
}

fn parse() -> (Vec<Number>, Vec<Symbol>) {
    let re = Regex::new(r"(\d+|[^.0-9])").unwrap();

    let lines = include_str!("inputs/I03.txt").lines();

    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];

    lines.enumerate().for_each(|(line_number, line)| {
        re.find_iter(line).for_each(|res_match| {
            let value = res_match.as_str().parse::<u32>();

            match value {
                Ok(val) => numbers.push(Number {
                    value: val,
                    line: line_number as i32,
                    start: res_match.start() as i32,
                    end: res_match.end() as i32,
                    active: false,
                }),
                Err(_) => symbols.push(Symbol(
                    res_match.start() as i32,
                    line_number as i32,
                    res_match.as_str() == "*",
                )),
            }
        })
    });

    return (numbers, symbols);
}

fn is_adjacent(symbol: &Symbol, number: &Number) -> bool {
    //above or below the number
    if ((number.line == symbol.1 - 1) || (number.line == symbol.1 + 1))
        && (symbol.0 >= number.start - 1)
        && (symbol.0 <= number.end)
    {
        return true;
    }

    //next to the number
    if (number.line == symbol.1) && ((symbol.0 == number.start - 1) || (symbol.0 == number.end)) {
        return true;
    }

    return false;
}

pub const SOL03: Solution = Solution();
