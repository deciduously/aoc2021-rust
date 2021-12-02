use crate::get_input;
use anyhow::{anyhow, Result};
use std::str::FromStr;

pub fn run() -> Result<()> {
	let steps = parse_input(&get_input(2, 1)?)?;
	println!("part 1: {}", part_one(&steps));
	println!("part 2: {}", part_two(&steps));
	Ok(())
}

#[derive(Clone, Copy)]
enum Direction {
	Forward,
	Down,
	Up,
}

impl FromStr for Direction {
	type Err = std::io::Error;
	fn from_str(s: &str) -> Result<Direction, Self::Err> {
		match s.to_lowercase().as_str() {
			"forward" => Ok(Direction::Forward),
			"down" => Ok(Direction::Down),
			"up" => Ok(Direction::Up),
			_ => Err(std::io::Error::new(
				std::io::ErrorKind::InvalidInput,
				"Unsupported direction",
			)),
		}
	}
}

#[derive(Clone, Copy)]
struct Step {
	direction: Direction,
	amount: i32,
}

impl FromStr for Step {
	type Err = anyhow::Error;
	fn from_str(s: &str) -> Result<Step, Self::Err> {
		let split = s.split(' ').collect::<Vec<&str>>();
		if split.len() != 2 {
			return Err(anyhow!("Must have two components, received {}", s));
		}
		let direction = Direction::from_str(&split[0])?;
		let amount = split[1].parse()?;
		Ok(Step { direction, amount })
	}
}

#[derive(Debug, Default)]
struct Submarine {
	aim: i32,
	horizontal: i32,
	depth: i32,
}

impl Submarine {
	fn new() -> Self {
		Self::default()
	}

	fn simple_step(&mut self, step: Step) {
		match step.direction {
			Direction::Up => self.depth -= step.amount,
			Direction::Down => self.depth += step.amount,
			Direction::Forward => self.horizontal += step.amount,
		}
	}

	fn step_with_aim(&mut self, step: Step) {
		match step.direction {
			Direction::Up => self.aim -= step.amount,
			Direction::Down => self.aim += step.amount,
			Direction::Forward => {
				self.horizontal += step.amount;
				self.depth += self.aim * step.amount;
			},
		}
	}
}

fn parse_input(input: &str) -> Result<Vec<Step>> {
	let mut ret = Vec::new();
	for line in input.lines() {
		if !line.is_empty() {
			ret.push(Step::from_str(line)?);
		}
	}
	Ok(ret)
}

fn part_one(steps: &[Step]) -> i32 {
	let mut submarine = Submarine::new();
	for step in steps {
		submarine.simple_step(*step);
	}
	submarine.horizontal * submarine.depth
}

fn part_two(steps: &[Step]) -> i32 {
	let mut submarine = Submarine::new();
	for step in steps {
		submarine.step_with_aim(*step);
	}
	submarine.horizontal * submarine.depth
}

#[cfg(test)]
mod test {
	use super::*;
	use pretty_assertions::assert_eq;
	#[test]
	fn test_part_one() {
		let input = r#"
forward 5
down 5
forward 8
up 3
down 8
forward 2
"#;
		assert_eq!(part_one(&parse_input(input).unwrap()), 150);
	}
	#[test]
	fn test_part_two() {
		let input = r#"
forward 5
down 5
forward 8
up 3
down 8
forward 2
"#;
		assert_eq!(part_two(&parse_input(input).unwrap()), 900);
	}
}
