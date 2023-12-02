use std::str::FromStr;

advent_of_code::solution!(2);

struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;

        for cube_str in s.split(',') {
            let (number_str, color_str) = cube_str
                .trim()
                .split_once(' ')
                .ok_or(format!("Failed to split cube string: {cube_str}"))?;

            let number = number_str
                .parse::<u32>()
                .map_err(|e| format!("round number parse: {}", e.to_string()))?;

            match color_str {
                "blue" => blue = number,
                "red" => red = number,
                "green" => green = number,
                _ => Err(format!("Unknown color: {color_str}"))?,
            }
        }

        Ok(Round { red, green, blue })
    }
}

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl FromStr for Game {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (str_game, str_rounds) = line
            .split_once(": ")
            .ok_or("Failed to split game and rounds")?;

        let str_game_id = str_game
            .split_whitespace()
            .skip(1)
            .next()
            .ok_or("Failed to split 'Game ' from number")?;

        let id = str_game_id
            .parse::<u32>()
            .map_err(|e| format!("game id parse: {}", e.to_string()))?;

        let rounds: Vec<Round> = str_rounds
            .split(';')
            .map(|str_round| Round::from_str(str_round))
            .collect::<Result<Vec<Round>, _>>()?;

        Ok(Game { id, rounds })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| Game::from_str(line).unwrap())
            .filter(|game| {
                game.rounds
                    .iter()
                    .all(|round| round.red <= 12 && round.green <= 13 && round.blue <= 14)
            })
            .fold(0, |acc, g| acc + g.id),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| Game::from_str(line).unwrap())
            .map(|game| {
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;
                for round in game.rounds {
                    red = red.max(round.red);
                    green = green.max(round.green);
                    blue = blue.max(round.blue);
                }
                red * green * blue
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
