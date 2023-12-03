use anyhow::{anyhow, Result};
use log::info;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct NumWithPosition {
    value: usize,
    position: Position,
    length: usize,
}

#[derive(Debug, Default)]
struct EngineSchematic {
    grid: Vec<Vec<char>>,
    numbers: Vec<NumWithPosition>,
}

impl EngineSchematic {
    fn part_numbers(&self) -> Vec<NumWithPosition> {
        let mut part_numbers = Vec::new();

        'num_loop: for num in self.numbers.iter() {
            let pos = num.position;
            for i in 0..num.length {
                for (neighbor_x, neighbor_y) in
                    util::grid_neighbors(&self.grid, pos.x + i, pos.y, true)
                {
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
        .part_numbers()
        .into_iter()
        .map(|n| n.value)
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
}
