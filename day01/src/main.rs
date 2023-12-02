use anyhow::{anyhow, Result};
use log::info;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::OnceLock;
use util::MinMax;

static STRING_TO_DIGIT_MAP: OnceLock<HashMap<String, usize>> = OnceLock::new();

fn string_to_digit_map() -> &'static HashMap<String, usize> {
    STRING_TO_DIGIT_MAP.get_or_init(|| {
        let mut map = HashMap::new();
        for n in 1..10 {
            map.insert(n.to_string(), n);
            map.insert(
                match n {
                    1 => "one",
                    2 => "two",
                    3 => "three",
                    4 => "four",
                    5 => "five",
                    6 => "six",
                    7 => "seven",
                    8 => "eight",
                    9 => "nine",
                    _ => unreachable!(),
                }
                .to_string(),
                n,
            );
        }
        map
    })
}

#[derive(Debug)]
struct CalibrationValue(usize);

impl FromStr for CalibrationValue {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 0 = index, 1 = value
        let mut left_most: Option<(usize, &usize)> = None;
        let mut right_most: Option<(usize, &usize)> = None;

        for (n, v) in string_to_digit_map() {
            let min_max = s.match_indices(n).map(|m| m.0).collect::<MinMax>();

            if let Some(min) = min_max.min {
                if left_most.is_none() || min < left_most.unwrap().0 {
                    left_most = Some((min, v));
                }
            }

            if let Some(max) = min_max.max {
                if right_most.is_none() || max > right_most.unwrap().0 {
                    right_most = Some((max, v));
                }
            }
        }

        if left_most.is_none() || right_most.is_none() {
            return Err(anyhow!(
                "Digits not found in {} {:?} + {:?}",
                s,
                left_most,
                right_most
            ));
        }

        let mut val = left_most.unwrap().1 * 10;
        val += right_most.unwrap().1;

        Ok(CalibrationValue(val))
    }
}

fn main() -> Result<()> {
    let result: usize = util::input()?
        .into_iter()
        .map_while(|s| CalibrationValue::from_str(&s).ok())
        .map(|c| c.0)
        .sum();

    info!("Result: {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    impl PartialEq<CalibrationValue> for usize {
        fn eq(&self, other: &CalibrationValue) -> bool {
            *self == other.0
        }
    }

    #[test]
    fn test_from_str() -> Result<()> {
        util::init_test_logger()?;

        assert_eq!(29, CalibrationValue::from_str("two1nine")?);
        assert_eq!(83, CalibrationValue::from_str("eightwothree")?);
        assert_eq!(13, CalibrationValue::from_str("abcone2threexyz")?);
        assert_eq!(24, CalibrationValue::from_str("xtwone3four")?);
        assert_eq!(42, CalibrationValue::from_str("4nineeightseven2")?);
        assert_eq!(14, CalibrationValue::from_str("zoneight234")?);
        assert_eq!(76, CalibrationValue::from_str("7pqrstsixteen")?);
        assert_eq!(62, CalibrationValue::from_str("6twofive3two")?);

        Ok(())
    }
}
