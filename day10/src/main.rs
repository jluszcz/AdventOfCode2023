use anyhow::{anyhow, Result};
use log::{debug, info, trace};
use std::fmt::Debug;
use util::Neighbor;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthAndEast,
    NorthAndWest,
    SouthAndWest,
    SouthAndEast,
    Ground,
    Start,
}

impl TryFrom<char> for Pipe {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthAndEast,
            'J' => Self::NorthAndWest,
            '7' => Self::SouthAndWest,
            'F' => Self::SouthAndEast,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => return Err(anyhow!("Unknown pipe value: {}", value)),
        })
    }
}

impl From<&Pipe> for char {
    fn from(value: &Pipe) -> Self {
        match value {
            Pipe::Vertical => '|',
            Pipe::Horizontal => '-',
            Pipe::NorthAndEast => 'L',
            Pipe::NorthAndWest => 'J',
            Pipe::SouthAndWest => '7',
            Pipe::SouthAndEast => 'F',
            Pipe::Ground => '.',
            Pipe::Start => 'S',
        }
    }
}

impl Debug for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vertical => write!(f, "Vertical:{}", char::from(self)),
            Self::Horizontal => write!(f, "Horizontal:{}", char::from(self)),
            Self::NorthAndEast => write!(f, "NorthAndEast:{}", char::from(self)),
            Self::NorthAndWest => write!(f, "NorthAndWest:{}", char::from(self)),
            Self::SouthAndWest => write!(f, "SouthAndWest:{}", char::from(self)),
            Self::SouthAndEast => write!(f, "SouthAndEast:{}", char::from(self)),
            Self::Ground => write!(f, "Ground:{}", char::from(self)),
            Self::Start => write!(f, "Start:{}", char::from(self)),
        }
    }
}

struct Map {
    grid: Vec<Vec<Pipe>>,
    start: (usize, usize),
}

impl Map {
    fn _dump(&self, highlight: (usize, usize)) {
        for (y, line) in self.grid.iter().enumerate() {
            let mut output = String::new();
            for (x, line) in line.iter().enumerate() {
                if (x, y) == highlight {
                    output.push('*');
                } else {
                    output.push(char::from(line));
                }
            }
            info!("{}", output);
        }
    }

    fn cycle_len(&self) -> usize {
        let mut len = 0;

        let mut curr = self.start;
        let mut prev = None;
        loop {
            let next = self.next(curr, &prev);
            len += 1;

            if next == self.start {
                return 1 + len / 2;
            }

            prev = Some(curr);
            curr = next;
        }
    }

    fn next(&self, from: (usize, usize), prev: &Option<(usize, usize)>) -> (usize, usize) {
        let (x, y) = from;

        let neighbors = util::grid_neighbors(&self.grid, x, y, false)
            .into_iter()
            .filter_map(|n| {
                let (n_x, n_y) = n.into();
                let pipe = self.grid[n_y][n_x];
                if pipe == Pipe::Ground {
                    None
                } else if let Some((p_x, p_y)) = prev {
                    // Don't backtrack
                    if *p_x == n_x && *p_y == n_y {
                        None
                    } else {
                        Some((pipe, n))
                    }
                } else {
                    Some((pipe, n))
                }
            })
            .collect::<Vec<_>>();

        let current = self.grid[y][x];

        for (neighbor_pipe, neighbor) in neighbors {
            match (current, neighbor, neighbor_pipe) {
                // Moving into and out of the start is always allowed
                (Pipe::Start, _, _) => return neighbor.into(),

                // Ground pipes were filtered out
                (Pipe::Ground, _, _) | (_, _, Pipe::Ground) => unreachable!(),

                // Diagonal moves are not valid
                (
                    _,
                    Neighbor::UpperLeft(_, _)
                    | Neighbor::UpperRight(_, _)
                    | Neighbor::LowerLeft(_, _)
                    | Neighbor::LowerRight(_, _),
                    _,
                ) => unreachable!(),

                // Move out of a vertical pipe
                (Pipe::Vertical, Neighbor::Left(_, _) | Neighbor::Right(_, _), _) => continue,
                (
                    Pipe::Vertical,
                    Neighbor::Upper(_, _) | Neighbor::Lower(_, _),
                    Pipe::Vertical
                    | Pipe::NorthAndEast
                    | Pipe::NorthAndWest
                    | Pipe::SouthAndEast
                    | Pipe::SouthAndWest
                    | Pipe::Start,
                ) => return neighbor.into(),

                // Move out of a horizontal pipe
                (Pipe::Horizontal, Neighbor::Upper(_, _) | Neighbor::Lower(_, _), _) => continue,
                (
                    Pipe::Horizontal,
                    Neighbor::Left(_, _),
                    Pipe::Horizontal | Pipe::NorthAndEast | Pipe::SouthAndEast | Pipe::Start,
                ) => return neighbor.into(),
                (
                    Pipe::Horizontal,
                    Neighbor::Right(_, _),
                    Pipe::Horizontal | Pipe::NorthAndWest | Pipe::SouthAndWest | Pipe::Start,
                ) => return neighbor.into(),

                // Move out of a north/east pipe
                (Pipe::NorthAndEast, Neighbor::Left(_, _) | Neighbor::Lower(_, _), _) => continue,
                (
                    Pipe::NorthAndEast,
                    Neighbor::Upper(_, _),
                    Pipe::Vertical | Pipe::SouthAndWest | Pipe::SouthAndEast,
                ) => return neighbor.into(),
                (
                    Pipe::NorthAndEast,
                    Neighbor::Right(_, _),
                    Pipe::Horizontal | Pipe::NorthAndWest | Pipe::SouthAndWest | Pipe::Start,
                ) => return neighbor.into(),

                // Move out of a north/west pipe
                (Pipe::NorthAndWest, Neighbor::Left(_, _) | Neighbor::Upper(_, _), Pipe::Start) => {
                    return neighbor.into()
                }
                (Pipe::NorthAndWest, Neighbor::Right(_, _) | Neighbor::Lower(_, _), _) => continue,
                (
                    Pipe::NorthAndWest,
                    Neighbor::Upper(_, _),
                    Pipe::Vertical | Pipe::SouthAndWest | Pipe::SouthAndEast,
                ) => return neighbor.into(),
                (
                    Pipe::NorthAndWest,
                    Neighbor::Left(_, _),
                    Pipe::Horizontal | Pipe::NorthAndEast | Pipe::SouthAndEast,
                ) => return neighbor.into(),

                // Move out of a south/east pipe
                (
                    Pipe::SouthAndEast,
                    Neighbor::Right(_, _) | Neighbor::Lower(_, _),
                    Pipe::Start,
                ) => return neighbor.into(),
                (Pipe::SouthAndEast, Neighbor::Left(_, _) | Neighbor::Upper(_, _), _) => continue,
                (
                    Pipe::SouthAndEast,
                    Neighbor::Lower(_, _),
                    Pipe::Vertical | Pipe::NorthAndWest | Pipe::NorthAndEast,
                ) => return neighbor.into(),
                (
                    Pipe::SouthAndEast,
                    Neighbor::Right(_, _),
                    Pipe::Horizontal | Pipe::NorthAndWest | Pipe::SouthAndWest,
                ) => return neighbor.into(),

                // Move out of a south/west pipe
                (Pipe::SouthAndWest, Neighbor::Left(_, _) | Neighbor::Lower(_, _), Pipe::Start) => {
                    return neighbor.into()
                }
                (Pipe::SouthAndWest, Neighbor::Right(_, _) | Neighbor::Upper(_, _), _) => continue,
                (
                    Pipe::SouthAndWest,
                    Neighbor::Lower(_, _),
                    Pipe::Vertical | Pipe::NorthAndEast | Pipe::NorthAndWest,
                ) => return neighbor.into(),
                (
                    Pipe::SouthAndWest,
                    Neighbor::Left(_, _),
                    Pipe::Horizontal | Pipe::NorthAndEast | Pipe::SouthAndEast,
                ) => return neighbor.into(),

                (_, _, _) => todo!("{:?} {:?} {:?}", current, neighbor, neighbor_pipe),
            }
        }
        unreachable!("exhausted")
    }
}

impl TryFrom<Vec<String>> for Map {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut grid = Vec::with_capacity(value.len());

        let mut start = None;
        for (y, line) in value.into_iter().enumerate() {
            let mut pipes = Vec::with_capacity(line.len());

            for (x, c) in line.chars().enumerate() {
                let pipe = Pipe::try_from(c)?;
                if pipe == Pipe::Start {
                    match start {
                        Some(_) => return Err(anyhow!("Two start positions found")),
                        None => start = Some((x, y)),
                    }
                }
                pipes.push(pipe);
            }

            grid.push(pipes);
        }

        let map = Map {
            grid,
            start: start.ok_or_else(|| anyhow!("No start position found"))?,
        };
        trace!("{:?}", map);

        Ok(map)
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Start: {:?}", self.start)?;
        for line in self.grid.iter() {
            let line: String = line.iter().map(char::from).collect();
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let map = Map::try_from(util::init()?)?;
    debug!("{:?}", map);

    let result = map.cycle_len();

    info!("Result: {result}");

    Ok(())
}
