use anyhow::{anyhow, Result};
use log::{info, trace};
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

#[derive(Debug, Clone, Copy)]
struct Range {
    start: usize,
    len: usize,
}

#[derive(Debug)]
struct MappingRange {
    from: usize,
    to: usize,
    len: usize,
}

impl MappingRange {
    fn adjust(&self, value: usize) -> usize {
        if self.from > self.to {
            value - (self.from - self.to)
        } else {
            value + (self.to - self.from)
        }
    }

    fn transform(&self, value: &Range) -> Option<Vec<Range>> {
        let mut updated_ranges = Vec::new();

        if value.start > self.from + self.len || value.start + value.len < self.from {
            // The given value does not intersect with this mapping range, nothing is transformed
            return None;
        }

        let value_end = value.start + value.len;
        let mapping_end = self.from + self.len;

        if value.start >= self.from && value_end <= mapping_end {
            // The given value is fully contained within the mapping range, transform everything
            updated_ranges.push(Range {
                start: self.adjust(value.start),
                len: value.len,
            })
        } else if value.start >= self.from {
            // The start of the given value's range intersects with the mapping range
            // value:            [    ]
            // mapping_range: [    ]
            //
            let intersect_len = mapping_end - value.start;
            updated_ranges.push(Range {
                start: self.adjust(value.start + intersect_len),
                len: intersect_len,
            });
            updated_ranges.push(Range {
                start: value.start + intersect_len,
                len: value.len - intersect_len,
            });
        } else {
            // The end of the given value's range intersects with the mapping range
            // value:         [    ]
            // mapping_range:    [    ]
            //
            let intersect_len = self.from - value.start;
            updated_ranges.push(Range {
                start: value.start,
                len: value.len - intersect_len,
            });
            updated_ranges.push(Range {
                start: self.adjust(value.start + intersect_len),
                len: intersect_len,
            });
        }

        Some(updated_ranges)
    }
}

impl FromStr for MappingRange {
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

        Ok(MappingRange { from, to, len })
    }
}

#[derive(Debug)]
struct Mapping {
    from: Entry,
    to: Entry,
    entries: Vec<MappingRange>,
}

impl Mapping {
    fn get(&self, range: &Range) -> Option<Vec<Range>> {
        for m in &self.entries {
            let result = m.transform(range);
            if result.is_some() {
                return result;
            }
        }
        None
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<Range>,
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

    fn map(&self, range: &Range, from: Entry) -> Result<(Entry, Vec<Range>)> {
        let mapping = self
            .mappings
            .get(&from)
            .ok_or_else(|| anyhow!("Failed to find mapping for {:?}", from))?;

        let result = mapping.get(range).unwrap_or_else(|| vec![*range]);
        trace!("Mapped {:?} to {:?} with {:?}", range, result, mapping);

        Ok((mapping.to, result))
    }

    fn seed_range_to_min_location(&self) -> Result<usize> {
        let mut entry = Entry::Seed;
        let mut ranges = self.seeds.clone();

        loop {
            let mut next_entry = entry;
            let mut next_ranges = Vec::new();

            for range in ranges.iter() {
                let mut updated_ranges;

                (next_entry, updated_ranges) = self.map(range, entry)?;

                next_ranges.append(&mut updated_ranges);
            }

            entry = next_entry;
            ranges = next_ranges;

            if entry == Entry::Location {
                break;
            }
        }

        ranges.sort_by(|x, y| x.start.cmp(&y.start));

        Ok(ranges[0].start)
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

                let seed_ranges = seed_s
                    .split_ascii_whitespace()
                    .filter_map(|s| usize::from_str(s).ok())
                    .collect::<Vec<_>>();

                for n in (0..seed_ranges.len()).step_by(2) {
                    seeds.push(Range {
                        start: seed_ranges[n],
                        len: seed_ranges[n + 1],
                    });
                }

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
                mapping.entries.push(MappingRange::from_str(&line)?);
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
    let result = almanac.seed_range_to_min_location()?;

    info!("Result: {}", result);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mappings_single_seed() -> Result<()> {
        let mut almanac = Almanac::try_from(util::test_input()?)?;
        almanac.seeds = vec![Range { start: 82, len: 1 }];

        assert_eq!(46, almanac.seed_range_to_min_location()?);

        Ok(())
    }

    #[test]
    fn test_mappings() -> Result<()> {
        let almanac = Almanac::try_from(util::test_input()?)?;

        assert_eq!(46, almanac.seed_range_to_min_location()?);

        Ok(())
    }
}
