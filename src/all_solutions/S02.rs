use core::num;

use super::solutions::Solvable;

pub struct Solution();

pub const SOL02: Solution = Solution();

enum Color {
    Red,
    Green,
    Blue,
}

struct Cube {
    color: Color,
    number: u32,
}

struct Game {
    id: u32,
    cubes: Vec<Cube>
}


struct CubesDetails{
    red: Vec<u32>,
    green: Vec<u32>,
    blue: Vec<u32>,
}

impl Solvable for Solution{

    fn first_solution(&self) -> String {
        
        let games = parse_games();

        let result: u32 = games
        .into_iter()
        .map(|mut game| {
            for cube in game.cubes{
                match cube.color{
                    Color::Red => {if cube.number > 12 { game.id = 0;}},
                    Color::Green => {if cube.number > 13  { game.id = 0}},
                    Color::Blue => {if cube.number > 14  { game.id = 0}},
                }
            }
                game.id
        }).sum();

        return result.to_string();
    }

    fn second_solution(&self) -> String {
        
        let games = parse_games();

        let details = games
            .into_iter()
            .map(|game| {
                let mut detail = CubesDetails{
                    red: vec![],
                    green: vec![],
                    blue: vec![],
                };

                for cube in game.cubes{
                    match cube.color{
                        Color::Red => detail.red.push(cube.number),
                        Color::Green => detail.green.push(cube.number),
                        Color::Blue => detail.blue.push(cube.number),
                    }
                }

                detail
            });

        let result: u32 = details
            .map(|detail| {
                detail.red.iter().max().unwrap() * detail.green.iter().max().unwrap() * detail.blue.iter().max().unwrap()
            }).sum();

        result.to_string()
    }
}

fn parse_games() -> Vec<Game>{
    let lines = include_str!("inputs/I02.txt").lines();

    lines.map(|line| {
        let mut line_iter = line.split(":");
        let game_id: u32 = line_iter
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let cubes: Vec<Cube> = line_iter
            .next()
            .unwrap()
            .replace(";", ",")
            .replace(" ", "")
            .as_str()
            .split(",")
            .map(|str_cube| {
                let number_of_cubes: u32 = str_cube
                    .chars()
                    .filter(|c| c.is_digit(10))
                    .collect::<String>()
                    .parse()
                    .unwrap();
                
                let cube_type: String = str_cube
                    .chars()
                    .filter(|c| c.is_alphabetic())
                    .collect();

                Cube{
                    color: match cube_type.as_str() {
                        "red" => Color::Red,
                        "green" => Color::Green,
                        "blue" => Color::Blue,
                        _ => unreachable!()
                    },
                    number: number_of_cubes
                }
            }).collect();
        
            Game{
                id: game_id,
                cubes: cubes
            }
        
    }).collect()

}
