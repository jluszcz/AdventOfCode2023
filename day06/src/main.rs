use anyhow::{anyhow, Result};
use log::info;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn ways_to_break_record(&self) -> usize {
        (1..self.time)
            .filter(|hold_time| hold_time * (self.time - hold_time) > self.distance)
            .count()
    }
}

#[derive(Debug, Default)]
struct Races(Vec<Race>);

impl TryFrom<Vec<String>> for Races {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut races = Races::default();

        let mut times = Vec::new();
        let mut distances = Vec::new();

        for line in value {
            let target;
            if line.starts_with("Time:") {
                target = &mut times;
            } else if line.starts_with("Distance:") {
                target = &mut distances;
            } else {
                return Err(anyhow!("Invalid line: {}", line));
            }

            let (_, values) = line.split_once(':').unwrap();
            target.push(usize::from_str(&values.replace(' ', ""))?);
        }

        if times.len() != distances.len() {
            return Err(anyhow!(
                "Invalid times ({:?}) and distances ({:?})",
                times,
                distances,
            ));
        }

        for (time, distance) in times.into_iter().zip(distances.into_iter()) {
            races.0.push(Race { time, distance });
        }

        Ok(races)
    }
}

fn main() -> Result<()> {
    let result = Races::try_from(util::input()?)?
        .0
        .into_iter()
        .map(|r| r.ways_to_break_record())
        .reduce(|acc, n| acc * n)
        .unwrap();

    info!("Result: {}", result);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parsing() -> Result<()> {
        let races = Races::try_from(util::test_input()?)?;

        assert_eq!(1, races.0.len());

        assert_eq!(
            Race {
                time: 71530,
                distance: 940200
            },
            races.0[0]
        );

        Ok(())
    }

    #[test]
    fn test_ways_to_break_record() -> Result<()> {
        util::init_test_logger()?;

        let race = Race {
            time: 7,
            distance: 9,
        };

        assert_eq!(4, race.ways_to_break_record());

        Ok(())
    }
}
