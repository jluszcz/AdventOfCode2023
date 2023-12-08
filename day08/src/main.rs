use anyhow::{anyhow, Result};
use log::{debug, info, trace};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    L,
    R,
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::L),
            'R' => Ok(Self::R),
            _ => Err(anyhow!("Invalid direction: {}", value)),
        }
    }
}

#[derive(Debug)]
struct Node(String, String);

impl Node {
    fn next(&self, direction: &Direction) -> &String {
        match direction {
            Direction::L => &self.0,
            Direction::R => &self.1,
        }
    }
}

impl FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace('(', "");
        let s = s.replace(')', "");

        let (from, to) = s
            .split_once(", ")
            .ok_or_else(|| anyhow!("Failed to split {}", s))?;

        Ok(Self(from.to_string(), to.to_string()))
    }
}

#[derive(Debug)]
struct Map {
    directions: Vec<Direction>,
    nodes: HashMap<String, Node>,
}

impl Map {
    fn steps(&self, start: &str) -> Result<usize> {
        let mut steps = 0;

        let mut curr = start;
        for direction in self.directions.iter().cycle() {
            let next = self
                .nodes
                .get(curr)
                .ok_or_else(|| anyhow!("No next node for {curr}"))?
                .next(direction);

            steps += 1;

            trace!("{} + {:?} --> {}", curr, direction, next);
            curr = next;
            if curr.ends_with('Z') {
                return Ok(steps);
            }
        }

        unreachable!()
    }

    fn ghost_steps(&self) -> Result<usize> {
        let start_nodes = self
            .nodes
            .keys()
            .filter(|n| n.ends_with('A'))
            .collect::<Vec<_>>();

        let mut steps = vec![0; start_nodes.len()];

        debug!("Starting nodes: {:?}", start_nodes);

        for i in 0..start_nodes.len() {
            steps[i] = self.steps(start_nodes[i])?;
            debug!("Found path for {} in {} steps", start_nodes[i], steps[i]);
        }

        steps
            .into_iter()
            .reduce(util::least_common_multiple)
            .ok_or_else(|| anyhow!("Failed to find LCM"))
    }
}

impl TryFrom<Vec<String>> for Map {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut directions = Vec::new();
        let mut nodes = HashMap::new();

        for line in value.into_iter() {
            if line.is_empty() {
                continue;
            }

            if directions.is_empty() {
                directions = line
                    .chars()
                    .map_while(|d| Direction::try_from(d).ok())
                    .collect();
                continue;
            }

            let (id, node) = line
                .split_once(" = ")
                .ok_or_else(|| anyhow!("Failed to split {}", line))?;

            nodes.insert(id.to_string(), Node::from_str(node)?);
        }

        let map = Map { directions, nodes };
        trace!("{:?}", map);
        Ok(map)
    }
}

fn main() -> Result<()> {
    let map = Map::try_from(util::init()?)?;

    let result = map.ghost_steps()?;

    info!("Result: {result}");

    Ok(())
}
