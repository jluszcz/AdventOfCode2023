use std::str::FromStr;

use anyhow::{anyhow, Result};
use log::info;

#[derive(Debug)]
struct CalibrationValue(u32);

impl FromStr for CalibrationValue {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut val = String::new();

        for c in s.chars() {
            if c.is_numeric() {
                val.push(c);
                break;
            }
        }

        for c in s.chars().rev() {
            if c.is_numeric() {
                val.push(c);
                break;
            }
        }

        if val.len() != 2 {
            return Err(anyhow!("Not enough digits in {}", s));
        }

        let val = u32::from_str(&val)?;
        Ok(CalibrationValue(val))
    }
}

fn main() -> Result<()> {
    let result: u32 = util::input()?
        .into_iter()
        .map_while(|s| CalibrationValue::from_str(&s).ok())
        .map(|c| c.0)
        .sum();

    info!("Result: {}", result);

    Ok(())
}
