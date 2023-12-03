use anyhow::{anyhow, Result};
use log::info;
use std::collections::HashSet;
use std::str::FromStr;
use util::grid_neighbors;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Gear {
    part_numbers: HashSet<NumWithPosition>,
}

impl Gear {
    fn ratio(&self) -> usize {
        assert_eq!(2, self.part_numbers.len(), "Invalid gear: {:?}", self);
        self.part_numbers
            .iter()
            .map(|n| n.value)
            .reduce(|acc, n| acc * n)
            .unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct NumWithPosition {
    value: usize,
    position: Position,
    length: usize,
}

impl NumWithPosition {
    fn intersects(&self, pos: &Position) -> bool {
        if pos.y == self.position.y {
            for i in 0..self.length {
                if pos.x == self.position.x + i {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Debug, Default)]
struct EngineSchematic {
    grid: Vec<Vec<char>>,
    numbers: Vec<NumWithPosition>,
}

impl EngineSchematic {
    fn part_numbers(&self) -> Vec<NumWithPosition> {
        let mut part_numbers = Vec::new();

        'num_loop: for num in &self.numbers {
            let pos = num.position;
            for i in 0..num.length {
                for (neighbor_x, neighbor_y) in grid_neighbors(&self.grid, pos.x + i, pos.y, true) {
                    let neighbor = self.grid[neighbor_y][neighbor_x];
                    if !neighbor.is_numeric() && neighbor != '.' {
                        part_numbers.push(*num);
                        continue 'num_loop;
                    }
                }
            }
        }

        part_numbers
    }

    fn gears(&self) -> Vec<Gear> {
        let part_numbers = self.part_numbers();

        let mut gears = Vec::new();

        for (y, line) in self.grid.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c == '*' {
                    let mut neighboring_part_nums = HashSet::new();
                    for (neighbor_x, neighbor_y) in grid_neighbors(&self.grid, x, y, true) {
                        for part_num in &part_numbers {
                            if part_num.intersects(&Position {
                                x: neighbor_x,
                                y: neighbor_y,
                            }) {
                                neighboring_part_nums.insert(*part_num);
                                break;
                            }
                        }
                    }
                    if neighboring_part_nums.len() == 2 {
                        gears.push(Gear {
                            part_numbers: neighboring_part_nums,
                        })
                    }
                }
            }
        }

        gears
    }

    fn add_num_with_position(
        &mut self,
        value: &mut String,
        position: &mut Option<Position>,
    ) -> Result<()> {
        self.numbers.push(NumWithPosition {
            value: usize::from_str(value)?,
            position: position
                .take()
                .ok_or_else(|| anyhow!("No position found for {}", value))?,
            length: value.len(),
        });

        *value = String::new();

        Ok(())
    }
}

impl TryFrom<Vec<String>> for EngineSchematic {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut engine_schematic = EngineSchematic::default();

        for (y, line) in value.into_iter().enumerate() {
            let mut value = String::new();
            let mut position = None;

            for (x, c) in line.chars().enumerate() {
                if c.is_numeric() {
                    if position.is_none() {
                        position = Some(Position { x, y })
                    }
                    value.push(c);
                } else if !value.is_empty() {
                    engine_schematic.add_num_with_position(&mut value, &mut position)?;
                }
            }

            engine_schematic.grid.push(line.chars().collect());
            if !value.is_empty() {
                engine_schematic.add_num_with_position(&mut value, &mut position)?;
            }
        }

        Ok(engine_schematic)
    }
}

fn main() -> Result<()> {
    let result: usize = EngineSchematic::try_from(util::input()?)?
        .gears()
        .into_iter()
        .map(|n| n.ratio())
        .sum();

    info!("Result: {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() -> Result<()> {
        let lines = vec!["467..114..".to_string()];

        let schematic = EngineSchematic::try_from(lines)?;

        assert_eq!(2, schematic.numbers.len());

        assert_eq!(
            NumWithPosition {
                value: 467,
                position: Position { x: 0, y: 0 },
                length: 3,
            },
            schematic.numbers[0]
        );

        assert_eq!(
            NumWithPosition {
                value: 114,
                position: Position { x: 5, y: 0 },
                length: 3,
            },
            schematic.numbers[1]
        );

        Ok(())
    }

    #[test]
    fn test_find_part_numbers() -> Result<()> {
        let schematic = EngineSchematic::try_from(util::test_input()?)?;

        let part_nums: Vec<usize> = schematic.part_numbers().iter().map(|n| n.value).collect();

        for expected_part_num in schematic.numbers.iter().map(|n| n.value) {
            if expected_part_num == 114 || expected_part_num == 58 {
                assert!(
                    !part_nums.contains(&expected_part_num),
                    "Found unexpected part num: {}",
                    expected_part_num
                );
            } else {
                assert!(
                    part_nums.contains(&expected_part_num),
                    "Did not find expected part num: {}",
                    expected_part_num
                );
            }
        }

        Ok(())
    }

    #[test]
    fn test_find_gears() -> Result<()> {
        let schematic = EngineSchematic::try_from(util::test_input()?)?;

        let gear_ratios: Vec<usize> = schematic.gears().iter().map(|n| n.ratio()).collect();

        assert_eq!(2, gear_ratios.len());
        assert!(gear_ratios.contains(&16345));
        assert!(gear_ratios.contains(&451490));

        Ok(())
    }
}
