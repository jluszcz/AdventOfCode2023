use anyhow::{anyhow, Result};
use log::{info, trace};
use std::str::FromStr;

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
}

impl Card {
    fn value(&self) -> usize {
        let winner_ct = self
            .winning_numbers
            .iter()
            .filter_map(|n| self.numbers.binary_search(n).ok())
            .count();

        if winner_ct > 0 {
            1 << (winner_ct - 1)
        } else {
            0
        }
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (_, s) = input
            .split_once(':')
            .ok_or_else(|| anyhow!("Failed to split {}", input))?;

        let (winning_numbers, numbers) = s
            .split_once('|')
            .ok_or_else(|| anyhow!("Failed to split {}", s))?;

        let winning_numbers = winning_numbers
            .split_ascii_whitespace()
            .map_while(|s| usize::from_str(s).ok())
            .collect();

        let mut numbers = numbers
            .split_ascii_whitespace()
            .map_while(|s| usize::from_str(s).ok())
            .collect::<Vec<_>>();
        numbers.sort();

        let card = Card {
            winning_numbers,
            numbers,
        };
        trace!("{} --> {:?}", input, card);
        Ok(card)
    }
}

fn main() -> Result<()> {
    let result: usize = util::input()?
        .into_iter()
        .map_while(|l| Card::from_str(&l).ok())
        .map(|c| c.value())
        .sum();

    info!("Result: {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() -> Result<()> {
        util::init_test_logger()?;

        let card = Card::from_str("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")?;

        assert_eq!(8, card.numbers.len());
        assert_eq!(5, card.winning_numbers.len());

        Ok(())
    }

    #[test]
    fn test_value() -> Result<()> {
        util::init_test_logger()?;

        let card = Card::from_str("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")?;

        assert_eq!(8, card.value());

        Ok(())
    }
}
