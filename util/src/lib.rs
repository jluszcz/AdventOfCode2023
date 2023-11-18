use anyhow::{anyhow, Result};
use env_logger::Target;
use log::{trace, LevelFilter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const INPUT_PATH: &str = "input/input";
const TEST_INPUT_PATH: &str = "input/example";

fn init_logger(level: LevelFilter) -> Result<()> {
    inner_init_logger(Some(level), false)
}

#[cfg(test)]
pub fn init_test_logger() -> Result<()> {
    inner_init_logger(Some(LevelFilter::Trace), true)
}

fn inner_init_logger(level: Option<LevelFilter>, is_test: bool) -> Result<()> {
    let _ = env_logger::builder()
        .target(Target::Stdout)
        .filter_level(level.unwrap_or(LevelFilter::Info))
        .is_test(is_test)
        .try_init();

    Ok(())
}

pub fn input() -> Result<Vec<String>> {
    init_logger(LevelFilter::Info)?;
    read_lines(INPUT_PATH)
}

pub fn test_input() -> Result<Vec<String>> {
    init_logger(LevelFilter::Trace)?;
    read_lines(TEST_INPUT_PATH)
}

fn read_lines(path: &'static str) -> Result<Vec<String>> {
    let lines: Vec<_> = BufReader::new(File::open(Path::new(path))?)
        .lines()
        .map_while(Result::ok)
        .inspect(|l| trace!("{}", l))
        .collect();

    if !lines.is_empty() {
        Ok(lines)
    } else {
        Err(anyhow!("No input: {}", path))
    }
}

pub fn grid_neighbors<T>(
    grid: &[Vec<T>],
    x: usize,
    y: usize,
    include_diagonal: bool,
) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    // Below
    {
        let y = y + 1;
        if grid.get(y).and_then(|r| r.get(x)).is_some() {
            neighbors.push((x, y));

            if include_diagonal {
                // Lower Right
                if grid[y].get(x + 1).is_some() {
                    neighbors.push((x + 1, y));
                }

                // Lower Left
                if let Some(x) = x.checked_sub(1) {
                    neighbors.push((x, y));
                }
            }
        }
    }

    // Above
    if let Some(y) = y.checked_sub(1) {
        neighbors.push((x, y));

        if include_diagonal {
            // Upper Right
            if grid[y].get(x + 1).is_some() {
                neighbors.push((x + 1, y));
            }

            // Upper Left
            if let Some(x) = x.checked_sub(1) {
                neighbors.push((x, y));
            }
        }
    }

    // Right
    if grid.get(y).and_then(|r| r.get(x + 1)).is_some() {
        neighbors.push((x + 1, y));
    }

    // Left
    if let Some(x) = x.checked_sub(1) {
        neighbors.push((x, y));
    }

    neighbors
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_neighbors() {
        let grid = vec![vec![0; 10]; 10];

        fn assert_eq_ignore_order(
            mut expected: Vec<(usize, usize)>,
            mut neighbors: Vec<(usize, usize)>,
        ) {
            expected.sort_unstable();
            neighbors.sort_unstable();
            assert_eq!(expected, neighbors);
        }

        assert_eq_ignore_order(vec![(1, 0), (0, 1)], grid_neighbors(&grid, 0, 0, false));

        assert_eq_ignore_order(
            vec![(1, 0), (0, 1), (1, 1)],
            grid_neighbors(&grid, 0, 0, true),
        );

        assert_eq_ignore_order(
            vec![(4, 0), (6, 0), (5, 1)],
            grid_neighbors(&grid, 5, 0, false),
        );

        assert_eq_ignore_order(
            vec![(4, 0), (6, 0), (5, 1), (4, 1), (6, 1)],
            grid_neighbors(&grid, 5, 0, true),
        );

        assert_eq_ignore_order(vec![(8, 0), (9, 1)], grid_neighbors(&grid, 9, 0, false));

        assert_eq_ignore_order(
            vec![(8, 0), (9, 1), (8, 1)],
            grid_neighbors(&grid, 9, 0, true),
        );

        assert_eq_ignore_order(
            vec![(0, 4), (0, 6), (1, 5)],
            grid_neighbors(&grid, 0, 5, false),
        );

        assert_eq_ignore_order(
            vec![(0, 4), (0, 6), (1, 5), (1, 4), (1, 6)],
            grid_neighbors(&grid, 0, 5, true),
        );

        assert_eq_ignore_order(vec![(0, 8), (1, 9)], grid_neighbors(&grid, 0, 9, false));

        assert_eq_ignore_order(
            vec![(0, 8), (1, 9), (1, 8)],
            grid_neighbors(&grid, 0, 9, true),
        );

        assert_eq_ignore_order(
            vec![(3, 4), (4, 3), (4, 5), (5, 4)],
            grid_neighbors(&grid, 4, 4, false),
        );

        assert_eq_ignore_order(
            vec![
                (3, 3),
                (3, 4),
                (3, 5),
                (4, 3),
                (4, 5),
                (5, 3),
                (5, 4),
                (5, 5),
            ],
            grid_neighbors(&grid, 4, 4, true),
        );

        assert_eq_ignore_order(vec![(9, 8), (8, 9)], grid_neighbors(&grid, 9, 9, false));

        assert_eq_ignore_order(
            vec![(8, 8), (9, 8), (8, 9)],
            grid_neighbors(&grid, 9, 9, true),
        );
    }
}
