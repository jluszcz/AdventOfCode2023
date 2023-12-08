use anyhow::{anyhow, Result};
use clap::{Arg, ArgAction, Command};
use env_logger::Target;
use log::{trace, LevelFilter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

const INPUT_PATH: &str = "input/input";
const TEST_INPUT_PATH: &str = "input/example";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Input {
    Test,
    Actual,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "test" => Ok(Self::Test),
            "actual" => Ok(Self::Actual),
            _ => Err(anyhow!("Invalid input type: {}", s)),
        }
    }
}

pub fn init() -> Result<Vec<String>> {
    let matches = Command::new("advent-of-code")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue)
                .help("increase log level from the default for the input type"),
        )
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .default_value("actual")
                .help(format!(
                    "input type, {:?} or {:?}",
                    Input::Test,
                    Input::Actual
                )),
        )
        .get_matches();

    let verbose = matches.get_flag("verbose");
    let input = matches
        .get_one::<String>("input")
        .map(|s| Input::from_str(s))
        .unwrap()?;

    let log_level = match (input, verbose) {
        (Input::Actual, false) => LevelFilter::Info,
        (Input::Actual, true) => LevelFilter::Debug,
        (Input::Test, false) => LevelFilter::Debug,
        (Input::Test, true) => LevelFilter::Trace,
    };

    init_logger(log_level)?;

    match input {
        Input::Actual => self::input(),
        Input::Test => self::test_input(),
    }
}

fn init_logger(level: LevelFilter) -> Result<()> {
    inner_init_logger(Some(level), false)
}

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
    read_lines(INPUT_PATH)
}

pub fn test_input() -> Result<Vec<String>> {
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

#[derive(Debug)]
pub struct MinMax {
    pub min: Option<usize>,
    pub max: Option<usize>,
}

impl FromIterator<usize> for MinMax {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut min = None;
        let mut max = None;

        for i in iter {
            min = match min {
                None => Some(i),
                Some(m) => Some(usize::min(m, i)),
            };
            max = match max {
                None => Some(i),
                Some(m) => Some(usize::max(m, i)),
            };
        }

        MinMax { min, max }
    }
}

pub fn greatest_common_divisor(a: usize, b: usize) -> usize {
    if b > a {
        greatest_common_divisor(b, a)
    } else if b == 0 {
        a
    } else {
        greatest_common_divisor(b, a % b)
    }
}

pub fn least_common_multiple(a: usize, b: usize) -> usize {
    (a * b) / greatest_common_divisor(a, b)
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

    #[test]
    fn test_greatest_common_divisor() {
        assert_eq!(6, greatest_common_divisor(48, 18));
    }
}
