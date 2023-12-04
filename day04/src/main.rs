use anyhow::{anyhow, Result};
use log::{info, trace};
use std::cell::OnceCell;
use std::str::FromStr;

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
    winner_ct: OnceCell<usize>,
    ct: usize,
}

impl Card {
    fn new(winning_numbers: Vec<usize>, numbers: Vec<usize>) -> Self {
        Self {
            winning_numbers,
            numbers,
            winner_ct: OnceCell::new(),
            ct: 1,
        }
    }

    fn winner_ct(&mut self) -> usize {
        *self.winner_ct.get_or_init(|| {
            self.winning_numbers
                .iter()
                .filter_map(|n| self.numbers.binary_search(n).ok())
                .count()
        })
    }

    fn winner(&mut self) -> bool {
        self.winner_ct() > 0
    }

    #[cfg(test)]
    fn value(&mut self) -> usize {
        if self.winner() {
            1 << (self.winner_ct() - 1)
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

        let card = Card::new(winning_numbers, numbers);
        trace!("{} --> {:?}", input, card);
        Ok(card)
    }
}

fn main() -> Result<()> {
    let mut cards = util::input()?
        .into_iter()
        .map(|l| Card::from_str(&l).unwrap())
        .collect::<Vec<_>>();

    for i in 0..cards.len() {
        if cards[i].winner() {
            trace!(
                "Card {} is a winner, copying the next {} cards",
                i + 1,
                cards[i].winner_ct()
            );

            let card_ct = cards[i].ct;

            for j in (i + 1)..(i + cards[i].winner_ct() + 1) {
                if let Some(c) = cards.get_mut(j) {
                    trace!("Copying card {} {} times", j + 1, card_ct);
                    c.ct += card_ct;
                }
            }
        }
    }

    let result: usize = cards.into_iter().map(|c| c.ct).sum();
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

        let mut card = Card::from_str("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")?;

        assert_eq!(8, card.value());

        Ok(())
    }
}
