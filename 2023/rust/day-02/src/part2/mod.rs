use std::str::FromStr;

const RED_CUBES: u32 = 12;
const GREEN_CUBES: u32 = 13;
const BLUE_CUBES: u32 = 14;
const GAME_DELIMITER: char = ':';
const GAME_ROUND_DELIMITER: char = ';';
const CUBE_DELIMITER: char = ',';

pub fn solve(input: &str) -> Result<String, String> {
    let result: u32 = input.lines().fold(
        0u32,
        |mut sum, line| -> u32 {
            let game = line.parse().unwrap_or(Game::default());
            sum += game.power();
            sum
        }
    );

    Ok(result.to_string())
}

#[derive(Default, Eq, PartialEq, Debug)]
pub(crate) struct Game {
    id: u32,
    rounds: Vec<GameRound>
}

impl Game {
    pub fn new(id: u32, rounds: Vec<GameRound>) -> Self {
        Self {
            id,
            rounds
        }
    }

    pub fn is_valid(&self, red: &u32, green: &u32, blue: &u32) -> bool {
        self.rounds.iter().all(|round| -> bool {
            round.is_valid(red, green, blue)
        })
    }

    pub fn power(&self) -> u32 {
        let (min_r, min_g, min_b) = self.rounds.iter().fold(
            (0u32, 0u32, 0u32),
            |(mut r, mut g, mut b), round| -> (u32, u32, u32) {
                if round.red > r {
                    r = round.red;
                }
                if(round.green > g) {
                    g = round.green;
                }
                if(round.blue > b) {
                    b = round.blue;
                }
                (r, g, b)
            }
        );
        let power = min_r * min_g * min_b;
        power
    }
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((game, rounds_str)) = s.split_once(GAME_DELIMITER) {
            let id_str = &game[5..];
            let id: u32 = id_str.parse::<u32>().map_err(|e| e.to_string())?;
            let rounds: Vec<GameRound> =
                rounds_str.split(GAME_ROUND_DELIMITER)
                    .map(|round_str| -> GameRound {
                        let round: GameRound = round_str.parse().unwrap_or(GameRound::default());
                        round
                    })
                    .collect();
            let game = Self {
                id,
                rounds
            };
            Ok(game)
        }
        else {
            Err("couldn't split game from rounds!".to_string())
        }
    }
}

#[derive(Default, Eq, PartialEq, Debug)]
pub(crate) struct GameRound {
    red: u32,
    green: u32,
    blue: u32
}

impl GameRound {
    pub fn new(red: u32, green: u32, blue: u32) -> Self {
        Self {
            red,
            green,
            blue
        }
    }

    pub fn is_valid(&self, red: &u32, green: &u32, blue: &u32) -> bool {
        self.red <= *red && self.green <= *green && self.blue <= *blue
    }
}

impl FromStr for GameRound {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (red, green, blue) = s.split(CUBE_DELIMITER).fold(
            (0u32, 0u32, 0u32),
            |(mut r, mut g, mut b), item| -> (u32, u32, u32) {
                if let Some((count_str, color_str)) = item.trim_start().split_once(' ') {
                    let count: u32 = count_str.parse().unwrap_or(0);
                    match color_str {
                        "red" => r += count,
                        "green" => g += count,
                        "blue" => b += count,
                        _ => ()
                    }
                }

                (r, g, b)
            }
        );

        let game_round = Self {
            red,
            green,
            blue
        };

        Ok(game_round)
    }
}

#[cfg(test)]
mod tests {
    use crate::part2::*;

    #[test]
    fn test_solve() -> Result<(), String> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
                           Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
                           Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
                           Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
                           Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("8", solve(input)?);
        Ok(())
    }

    #[test]
    fn test_game_from_str() -> Result<(), String> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = Game::new(
            1,
            vec![
                GameRound::new(4, 0, 3),
                GameRound::new(1, 2, 6),
                GameRound::new(0, 2, 0),
            ]
        );
        assert_eq!(expected, input.parse()?);
        Ok(())
    }

    #[test]
    fn test_game_is_valid() -> Result<(), String> {
        let instance = Game::new(
            1,
            vec![
                GameRound::new(4, 0, 3),
                GameRound::new(1, 2, 6),
                GameRound::new(0, 2, 0),
            ]
        );
        assert!(instance.is_valid(&RED_CUBES, &GREEN_CUBES, &BLUE_CUBES));
        assert!(!instance.is_valid(&0u32, &GREEN_CUBES, &BLUE_CUBES));
        Ok(())
    }

    #[test]
    fn test_game_power() -> Result<(), String> {
        let instance = Game::new(
            1u32,
            vec![
                GameRound::new(4, 0, 3),
                GameRound::new(1, 2, 6),
                GameRound::new(0, 2, 0),
            ]
        );
        assert_eq!(48u32, instance.power());
        Ok(())
    }

    #[test]
    fn test_game_round_from_str() -> Result<(), String> {
        let input = "3 blue, 4 red";
        let expected = GameRound::new(4, 0, 3);
        assert_eq!(expected, input.parse()?);
        Ok(())
    }

    #[test]
    fn test_round_is_valid() -> Result<(), String> {
        let instance = GameRound::new(0, 2, 0);
        assert!(instance.is_valid(&RED_CUBES, &GREEN_CUBES, &BLUE_CUBES));
        assert!(!instance.is_valid(&RED_CUBES, &0u32, &BLUE_CUBES));
        Ok(())
    }
}

