use std::fs;
use nom::{
    character::complete::{digit1, char, alpha1, line_ending},
    bytes::complete::tag,
    multi::separated_list1,
    IResult
};

fn main() {
    let data = read_file("data.txt").expect("Not able to read file.");

    let (_, games) = parse_input(&data).unwrap();
    
    let result = games.iter()
        .map(|game| game.get_game_power())
        .sum::<u32>();

    println!("{}", result);
}

#[derive(PartialEq, Debug)]
enum CubeCount {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[derive(PartialEq, Debug)]
struct Sample {
    value: Vec<CubeCount>
}

impl Sample {
    fn from(v: Vec<CubeCount>) -> Sample {
        Sample { value: v }
    }
}

#[derive(PartialEq, Debug)]
struct Game {
    id:u32,
    samples: Vec<Sample>,
}

impl Game {
    fn get_game_power(&self) -> u32 {
        let (mut max_r, mut max_g, mut max_b) = (0, 0, 0);

        for sample in &self.samples {
            for cube_count in &sample.value {
                match cube_count {
                    CubeCount::Red(n) => max_r = max_r.max(*n),
                    CubeCount::Green(n) => max_g = max_g.max(*n),
                    CubeCount::Blue(n) => max_b = max_b.max(*n),
                }
            }
        }

        max_r * max_g * max_b
    }
}

fn read_file(filepath: &str) -> std::io::Result<String> {
    fs::read_to_string(filepath)
}

// Example: "2 green"
fn parse_cube_count(input:&str) -> IResult<&str, CubeCount>{
    let (input, count) = digit1(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, color) = alpha1(input)?;

    let count = match count.parse::<u32>() {
        Ok(n) => n,
        Err(_) => panic!("Failed to parse count"),
    };
    
    match color {
        "red" => Ok((input, CubeCount::Red(count))),
        "green" => Ok((input, CubeCount::Green(count))),
        "blue" => Ok((input, CubeCount::Blue(count))),
        _ => panic!("Unknown color"),
    }
}

// Example: "5 blue, 4 red, 13 green"
fn parse_sample(s:&str) -> IResult<&str, Sample>{
    let (input, v) = separated_list1(tag(", "), parse_cube_count)(s)?;
    Ok((input, Sample::from(v)))
}

// Example: "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
fn parse_game(s:&str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(s)?;

    let (input, id) = digit1(input)?;
    let id = match id.parse::<u32>() {
        Ok(n) => n,
        Err(_) => panic!("Failed to parse Game id"),
    };

    let (input, _) = tag(": ")(input)?;
    let (input, samples) = separated_list1(tag("; "), parse_sample)(input)?;

    Ok((input, Game{ id, samples }))
}

fn parse_input(s:&str) -> IResult<&str, Vec<Game>> {
    separated_list1(line_ending, parse_game)(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cube_count() {
        let input = "5 red";
        let result = parse_cube_count(input);

        match result {
            Ok((_, CubeCount::Red(count))) => assert_eq!(count, 5),
            _ => panic!("Unexpected result: {:?}", result),
        }
    }

    #[test]
    fn test_parse_sample() {
        let input = "5 red, 3 green, 2 blue";
        let result = parse_sample(input);
        match result {
            Ok((_, sample)) => {
                let expected = Sample::from(vec![
                    CubeCount::Red(5),
                    CubeCount::Green(3),
                    CubeCount::Blue(2),
                ]);
                assert_eq!(sample, expected);
            }
            _ => panic!("Unexpected result: {:?}", result),
        }
    }

    #[test]
    fn test_parse_game() {
        let input = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = parse_game(input);

        match result {
            Ok((_, game)) => {
                let expected = Game { 
                    id: 5, 
                    samples: vec![
                        Sample::from(vec![
                            CubeCount::Red(6),
                            CubeCount::Blue(1),
                            CubeCount::Green(3),
                        ]),
                        Sample::from(vec![
                            CubeCount::Blue(2),
                            CubeCount::Red(1),
                            CubeCount::Green(2),
                        ]),
                    ],
                };
                assert_eq!(game, expected);
            },
            _ => panic!("Unexpected result: {:?}", result)
        }
    }

}