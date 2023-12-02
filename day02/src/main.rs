use anyhow::{anyhow, Result};
use log::{info, trace};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Reveal {
    red: usize,
    green: usize,
    blue: usize,
}

impl Reveal {
    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

impl FromStr for Reveal {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for entry in s.split(',') {
            let (count, color) = entry
                .trim()
                .split_once(' ')
                .ok_or_else(|| anyhow!("Failed to split {}", entry))?;

            let count = usize::from_str(count)?;
            match color {
                "red" => red = count,
                "green" => green = count,
                "blue" => blue = count,
                _ => return Err(anyhow!("Invalid color: {}", color)),
            };
        }

        let reveal = Reveal { red, green, blue };
        trace!("{} --> {:?}", s, reveal);
        Ok(reveal)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: usize,
    reveals: Vec<Reveal>,
}

impl Game {
    fn min_set(&self) -> Reveal {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for reveal in self.reveals.iter() {
            red = usize::max(red, reveal.red);
            green = usize::max(green, reveal.green);
            blue = usize::max(blue, reveal.blue);
        }

        Reveal { red, green, blue }
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game, reveal_s) = s
            .trim()
            .split_once(':')
            .ok_or_else(|| anyhow!("Failed to split {}", s))?;

        let (_, id) = game
            .split_once(' ')
            .ok_or_else(|| anyhow!("Failed to split {}", game))?;

        let id = usize::from_str(id)?;

        let reveals = reveal_s
            .split(';')
            .map_while(|r| Reveal::from_str(r).ok())
            .collect();

        let game = Game { id, reveals };
        trace!("{} --> {:?}", s, game);
        Ok(game)
    }
}

fn main() -> Result<()> {
    let result: usize = util::input()?
        .iter()
        .map_while(|g| Game::from_str(g).ok())
        .map(|g| g.min_set())
        .map(|r| r.power())
        .sum();

    info!("Result: {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reveal_from_str() -> Result<()> {
        util::init_test_logger()?;

        assert_eq!(
            Reveal {
                red: 4,
                green: 0,
                blue: 3,
            },
            Reveal::from_str("3 blue, 4 red")?
        );

        assert_eq!(
            Reveal {
                red: 1,
                green: 2,
                blue: 6,
            },
            Reveal::from_str("1 red, 2 green, 6 blue")?
        );

        assert_eq!(
            Reveal {
                red: 0,
                green: 2,
                blue: 0,
            },
            Reveal::from_str("2 green")?
        );

        Ok(())
    }

    #[test]
    fn test_game_from_str() -> Result<()> {
        util::init_test_logger()?;

        assert_eq!(
            Game {
                id: 1,
                reveals: vec![
                    Reveal {
                        red: 4,
                        green: 0,
                        blue: 3,
                    },
                    Reveal {
                        red: 1,
                        green: 2,
                        blue: 6,
                    },
                    Reveal {
                        red: 0,
                        green: 2,
                        blue: 0,
                    },
                ],
            },
            Game::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")?
        );

        Ok(())
    }

    #[test]
    fn test_power() {
        let game = Game {
            id: 1,
            reveals: vec![
                Reveal {
                    red: 4,
                    green: 0,
                    blue: 3,
                },
                Reveal {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                Reveal {
                    red: 0,
                    green: 2,
                    blue: 0,
                },
            ],
        };

        let min_set = game.min_set();
        assert_eq!(
            min_set,
            Reveal {
                red: 4,
                green: 2,
                blue: 6
            }
        );

        assert_eq!(48, min_set.power());
    }
}
