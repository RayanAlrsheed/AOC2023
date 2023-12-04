use regex::Regex;

use super::solutions::{Solution, Solvable};

#[derive(Debug)]
struct Game {
    winning_cards: Vec<usize>,
    playing_cards: Vec<usize>,
}

impl Solvable for Solution {
    fn first_solution(&self) -> String {
        let games = parse();

        let result = games
            .iter()
            .map(|game| {
                let mut points: usize = 0;

                for card in &game.playing_cards {
                    if game.winning_cards.contains(&card) {
                        if points == 0 {
                            points = 1
                        } else {
                            points *= 2;
                        }
                    }
                }
                points
            })
            .sum::<usize>();

        return result.to_string();
    }

    fn second_solution(&self) -> String {
        let games = parse();

        let mut number_of_cards = vec![1; games.len()];

        for (card_number, game) in games.iter().enumerate() {
            let mut number_of_matches: usize = game
                .playing_cards
                .iter()
                .map(|card| {
                    if game.winning_cards.contains(card) {
                        1
                    } else {
                        0
                    }
                })
                .sum();

            if number_of_matches + card_number > number_of_cards.len() {
                number_of_matches = number_of_cards.len();
            } else {
                number_of_matches = number_of_matches + card_number;
            }

            for index in card_number + 1..number_of_matches + 1 {
                number_of_cards[index] += number_of_cards[card_number]
            }
        }

        number_of_cards.iter().sum::<usize>().to_string()
    }
}

fn parse() -> Vec<Game> {
    let lines = include_str!("inputs/I04.txt").lines();
    let re = Regex::new(r"\d+").unwrap();

    lines
        .map(|line| {
            let mut game: Game = Game {
                winning_cards: vec![],
                playing_cards: vec![],
            };

            let mut deck = line.split(":").last().unwrap().split("|");

            game.winning_cards = re
                .find_iter(deck.next().unwrap())
                .map(|number| number.as_str().parse::<usize>().unwrap())
                .collect();
            game.playing_cards = re
                .find_iter(deck.next().unwrap())
                .map(|number| number.as_str().parse::<usize>().unwrap())
                .collect();

            game
        })
        .collect()
}

pub const SOL04: Solution = Solution();
