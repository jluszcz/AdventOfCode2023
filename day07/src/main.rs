use anyhow::{anyhow, Result};
use log::info;
use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Card {
    Ace,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl Card {
    fn rank(&self) -> usize {
        match self {
            Card::Ace => 14,
            Card::King => 13,
            Card::Queen => 12,
            Card::Ten => 10,
            Card::Nine => 9,
            Card::Eight => 8,
            Card::Seven => 7,
            Card::Six => 6,
            Card::Five => 5,
            Card::Four => 4,
            Card::Three => 3,
            Card::Two => 2,
            Card::Joker => 1,
        }
    }

    fn len() -> usize {
        12
    }
}

impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Joker,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => {
                return Err(anyhow!("Invalid card: {}", value));
            }
        })
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank().cmp(&other.rank())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    Pair,
    HighCard,
}

impl HandType {
    fn rank(&self) -> usize {
        match self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::Pair => 2,
            HandType::HighCard => 1,
        }
    }

    fn score(hand: &[Card; 5]) -> Self {
        let mut counter = vec![0; Card::len() + 1];
        for card in hand {
            counter[*card as usize] += 1;
        }

        let (most_common_idx, most_common_ct) = counter
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != Card::Joker as usize)
            .max_by_key(|(_, ct)| *ct)
            .unwrap();

        // Increaes the most common count by the count of wild jokers
        let most_common_ct = if most_common_idx != Card::Joker as usize {
            *most_common_ct + counter[Card::Joker as usize]
        } else {
            *most_common_ct
        };

        match most_common_ct {
            5 => Self::FiveOfAKind,
            4 => Self::FourOfAKind,
            3 | 2 => {
                let (_, next_most_common_ct) = counter
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != most_common_idx && *i != Card::Joker as usize)
                    .max_by_key(|(_, ct)| *ct)
                    .unwrap();

                match (most_common_ct, *next_most_common_ct) {
                    (3, 2) => Self::FullHouse,
                    (3, _) => Self::ThreeOfAKind,
                    (2, 2) => Self::TwoPair,
                    _ => Self::Pair,
                }
            }
            _ => Self::HighCard,
        }
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank().cmp(&other.rank())
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    hand: [Card; 5],
    hand_type: HandType,
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            return Err(anyhow!("Invalid hand: {}", s));
        }

        let mut hand: [Card; 5] = [Card::Ace; 5];
        for (i, c) in s.chars().enumerate() {
            hand[i] = Card::try_from(c)?;
        }

        let hand_type = HandType::score(&hand);
        Ok(Hand { hand, hand_type })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_type_ord = self.hand_type.cmp(&other.hand_type);
        match hand_type_ord {
            Ordering::Greater | Ordering::Less => hand_type_ord,
            Ordering::Equal => {
                for (self_card, other_card) in self.hand.iter().zip(other.hand.iter()) {
                    let card_ord = self_card.cmp(other_card);
                    match card_ord {
                        Ordering::Greater | Ordering::Less => {
                            return card_ord;
                        }
                        _ => {
                            continue;
                        }
                    }
                }
                Ordering::Equal
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct HandWithBid(Hand, usize);

impl FromStr for HandWithBid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s
            .split_once(' ')
            .ok_or_else(|| anyhow!("Failed to split {}", s))?;

        let hand = Hand::from_str(hand)?;
        let bid = usize::from_str(bid)?;

        Ok(HandWithBid(hand, bid))
    }
}

#[derive(Debug)]
struct Hands {
    hands: Vec<HandWithBid>,
}

impl TryFrom<Vec<String>> for Hands {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        Ok(Hands {
            hands: value
                .into_iter()
                .map_while(|s| HandWithBid::from_str(&s).ok())
                .collect(),
        })
    }
}

impl Hands {
    fn total_winnings(&mut self) -> usize {
        self.hands.sort_by(|a, b| a.0.cmp(&b.0));

        let mut total_winnings = 0;
        for (rank, bid) in self.hands.iter().map(|h| h.1).enumerate() {
            total_winnings += (rank + 1) * bid;
        }
        total_winnings
    }
}

fn main() -> Result<()> {
    let mut hands = Hands::try_from(util::input()?)?;

    let result = hands.total_winnings();

    info!("Result: {result}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_hand_type() -> Result<()> {
        util::init_test_logger()?;

        let hand = Hand::from_str("A2345")?;
        assert_eq!(HandType::HighCard, hand.hand_type);

        let hand = Hand::from_str("32T3K")?;
        assert_eq!(HandType::Pair, hand.hand_type);

        let hand = Hand::from_str("KK677")?;
        assert_eq!(HandType::TwoPair, hand.hand_type);

        let hand = Hand::from_str("QQQKA")?;
        assert_eq!(HandType::ThreeOfAKind, hand.hand_type);

        let hand = Hand::from_str("KQQQQ")?;
        assert_eq!(HandType::FourOfAKind, hand.hand_type);

        let hand = Hand::from_str("QQQQQ")?;
        assert_eq!(HandType::FiveOfAKind, hand.hand_type);

        Ok(())
    }

    #[test]
    fn test_parse_hand_type_with_jokers_wild() -> Result<()> {
        util::init_test_logger()?;

        let hand = Hand::from_str("T55J5")?;
        assert_eq!(HandType::FourOfAKind, hand.hand_type);

        let hand = Hand::from_str("KTJJT")?;
        assert_eq!(HandType::FourOfAKind, hand.hand_type);

        let hand = Hand::from_str("QQQJA")?;
        assert_eq!(HandType::FourOfAKind, hand.hand_type);

        let hand = Hand::from_str("QQJKK")?;
        assert_eq!(HandType::FullHouse, hand.hand_type);

        let hand = Hand::from_str("QQJ23")?;
        assert_eq!(HandType::ThreeOfAKind, hand.hand_type);

        let hand = Hand::from_str("JJJJJ")?;
        assert_eq!(HandType::FiveOfAKind, hand.hand_type);

        let hand = Hand::from_str("JJJJK")?;
        assert_eq!(HandType::FiveOfAKind, hand.hand_type);

        Ok(())
    }

    #[test]
    fn test_ordering() -> Result<()> {
        util::init_test_logger()?;

        let hand_a = Hand::from_str("33332")?;
        let hand_b = Hand::from_str("2AAAA")?;
        assert!(hand_a > hand_b);

        let hand_a = Hand::from_str("77888")?;
        let hand_b = Hand::from_str("77788")?;
        assert!(hand_a > hand_b);

        let hand_a = Hand::from_str("T55J5")?;
        let hand_b = Hand::from_str("KTJJT")?;
        assert!(hand_b > hand_a);

        Ok(())
    }
}
