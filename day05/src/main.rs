use anyhow::{anyhow, Result};
use log::{debug, info, trace};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Entry {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl FromStr for Entry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seed" => Ok(Self::Seed),
            "soil" => Ok(Self::Soil),
            "fertilizer" => Ok(Self::Fertilizer),
            "water" => Ok(Self::Water),
            "light" => Ok(Self::Light),
            "temperature" => Ok(Self::Temperature),
            "humidity" => Ok(Self::Humidity),
            "location" => Ok(Self::Location),
            _ => Err(anyhow!("Unknown type: {}", s)),
        }
    }
}

#[derive(Debug)]
struct Range {
    from: usize,
    to: usize,
    len: usize,
}

impl Range {
    fn transform(&self, value: &usize) -> Option<usize> {
        if (self.from..self.from + self.len).contains(value) {
            trace!(
                "{:?} [{}, {}) contains {}",
                self,
                self.from,
                self.from + self.len,
                value
            );
            Some(if self.from > self.to {
                value - (self.from - self.to)
            } else {
                value + (self.to - self.from)
            })
        } else {
            None
        }
    }
}

impl FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let m = s
            .split_ascii_whitespace()
            .map_while(|s| usize::from_str(s).ok())
            .collect::<Vec<_>>();

        if m.len() != 3 {
            return Err(anyhow! {"Failed to parse range: {}", s});
        }

        let from = m[1];
        let to = m[0];
        let len = m[2];

        Ok(Range { from, to, len })
    }
}

#[derive(Debug)]
struct Mapping {
    from: Entry,
    to: Entry,
    entries: Vec<Range>,
}

impl Mapping {
    fn get(&self, key: &usize) -> usize {
        for m in &self.entries {
            if let Some(v) = m.transform(key) {
                trace!("Mapped {} to {}", key, v);
                return v;
            }
        }
        *key
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    mappings: HashMap<Entry, Mapping>,
}

impl Almanac {
    fn parse_mapping_type(input: &str) -> Result<(Entry, Entry)> {
        let (t, _) = input
            .split_once(' ')
            .ok_or_else(|| anyhow!("Failed to split {}", input))?;

        let t = t
            .split('-')
            .filter_map(|s| Entry::from_str(s).ok())
            .collect::<Vec<_>>();

        if t.len() != 2 {
            return Err(anyhow!("Failed to parse {}", input));
        }

        Ok((t[0], t[1]))
    }

    fn map(&self, value: usize, from: Entry) -> Result<(Entry, usize)> {
        let mapping = self
            .mappings
            .get(&from)
            .ok_or_else(|| anyhow!("Failed to find mapping for {:?}", from))?;

        let next_val = mapping.get(&value);
        trace!(
            "Mapped {:?} {} to {:?} {}",
            value,
            next_val,
            mapping.to,
            next_val
        );

        Ok((mapping.to, next_val))
    }

    fn seed_to_location(&self, seed: usize) -> Result<usize> {
        let mut source = Entry::Seed;
        let mut value = seed;

        let value = loop {
            (source, value) = self.map(value, source)?;

            if source == Entry::Location {
                break value;
            }
        };
        debug!("Seed {} --> Location {}", seed, value);
        Ok(value)
    }
}

impl TryFrom<Vec<String>> for Almanac {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut seeds = Vec::new();
        let mut mappings = HashMap::new();

        let mut current_mapping: Option<Mapping> = None;

        for line in value {
            if seeds.is_empty() {
                let (title, seed_s) = line
                    .split_once(':')
                    .ok_or_else(|| anyhow!("Failed to split {}", line))?;

                if title != "seeds" {
                    return Err(anyhow!("Invalid seed line {}", line));
                }

                seeds = seed_s
                    .split_ascii_whitespace()
                    .filter_map(|s| usize::from_str(s).ok())
                    .collect::<Vec<_>>();

                continue;
            }

            if line.trim().is_empty() {
                if current_mapping.is_some() {
                    let mapping = current_mapping.take().unwrap();
                    mappings.insert(mapping.from, mapping);
                }
                continue;
            }

            if let Some(mapping) = current_mapping.as_mut() {
                mapping.entries.push(Range::from_str(&line)?);
            }

            if current_mapping.is_none() {
                let (from, to) = Self::parse_mapping_type(&line)?;
                trace!("Parsing {:?} to {:?} map", from, to);
                current_mapping = Some(Mapping {
                    from,
                    to,
                    entries: Vec::new(),
                });
            }
        }

        if current_mapping.is_some() {
            let mapping = current_mapping.take().unwrap();
            mappings.insert(mapping.from, mapping);
        }

        let almanac = Almanac { seeds, mappings };
        trace!("{:?}", almanac);
        Ok(almanac)
    }
}

fn main() -> Result<()> {
    let almanac = Almanac::try_from(util::input()?)?;
    let result = almanac
        .seeds
        .iter()
        .filter_map(|s| almanac.seed_to_location(*s).ok())
        .min()
        .ok_or_else(|| anyhow!("Failed to find minimum location"))?;

    info!("Result: {}", result);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mappings() -> Result<()> {
        let almanac = Almanac::try_from(util::test_input()?)?;

        assert_eq!((Entry::Soil, 81), almanac.map(79, Entry::Seed)?);
        assert_eq!((Entry::Fertilizer, 81), almanac.map(81, Entry::Soil)?);
        assert_eq!((Entry::Water, 81), almanac.map(81, Entry::Fertilizer)?);
        assert_eq!((Entry::Light, 74), almanac.map(81, Entry::Water)?);
        assert_eq!((Entry::Temperature, 78), almanac.map(74, Entry::Light)?);
        assert_eq!((Entry::Humidity, 78), almanac.map(78, Entry::Temperature)?);
        assert_eq!((Entry::Location, 82), almanac.map(78, Entry::Humidity)?);

        assert_eq!(82, almanac.seed_to_location(79)?);

        Ok(())
    }
}
