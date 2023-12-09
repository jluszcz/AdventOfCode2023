use anyhow::Result;
use log::{info, log_enabled, trace, Level};
use std::str::FromStr;

#[derive(Debug, Default)]
struct OasisReadings(Vec<isize>);

impl OasisReadings {
    fn prev_value(&self) -> isize {
        let mut history: Vec<Vec<isize>> = vec![self.0.clone()];

        let mut i = 0;
        loop {
            let curr = &history[i];

            let differences = (0..(curr.len() - 1))
                .map(|n| curr[n + 1] - curr[n])
                .collect();

            history.push(differences);

            if history[i + 1].iter().all(|n| *n == 0) {
                break;
            }

            i += 1;
        }

        if log_enabled!(Level::Trace) {
            for line in &history {
                trace!("{:?}", line);
            }
        }

        let mut prev = None;
        for i in (0..(history.len() - 1)).rev() {
            let next_history = &history[i];

            prev = Some(match prev {
                Some(prev) => next_history[0] - prev,
                None => {
                    let prev_history = &history[i + 1];
                    next_history[0] - prev_history[0]
                }
            })
        }

        prev.unwrap()
    }
}

impl FromStr for OasisReadings {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let readings = s
            .split_ascii_whitespace()
            .map_while(|i| isize::from_str(i).ok())
            .collect::<Vec<_>>();

        trace!("{:?}", readings);
        Ok(OasisReadings(readings))
    }
}

fn main() -> Result<()> {
    let result = util::init()?
        .into_iter()
        .map_while(|l| OasisReadings::from_str(&l).ok())
        .map(|r| r.prev_value())
        .sum::<isize>();

    info!("Result: {result}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_prev_value() -> Result<()> {
        util::init_test_logger()?;

        let readings = OasisReadings::from_str("0 3 6 9 12 15")?;
        assert_eq!(-3, readings.prev_value());

        let readings = OasisReadings::from_str("1 3 6 10 15 21")?;
        assert_eq!(0, readings.prev_value());

        let readings = OasisReadings::from_str("10 13 16 21 30 45")?;
        assert_eq!(5, readings.prev_value());

        Ok(())
    }
}
