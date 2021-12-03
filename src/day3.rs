use crate::get_input;
use anyhow::{anyhow, Result};

// TODO - explore the bitvec crate!

pub fn run() -> Result<()> {
	let input = get_input(3, 1)?;
	let parsed = parse_input(&input)?;
	println!("part one: {}", part_one(parsed));
	Ok(())
}

fn parse_input(input: &str) -> Result<Vec<Vec<bool>>> {
	let mut ret = Vec::new();
	for line in input.lines() {
		if !line.is_empty() {
			let mut num = Vec::new();
			for char in line.chars() {
				match char {
					'1' => num.push(true),
					'0' => num.push(false),
					_ => return Err(anyhow!("Unparsable number found with digit {}", char)),
				}
			}
			ret.push(num);
		}
	}
	Ok(ret)
}

// Transform a bool slice to an i32
//fn bools_to_binary(v: &[bool]) -> i32 {
//	v.iter()
//		.rev()
//		.map(|&el| if el { 1 } else { 0 })
//		.fold(0, |acc, b| acc * 2 + b as i32)
//}

fn bools_to_binary(v: &[bool]) -> i32 {
	let mut s = String::new();
	for el in v {
		if *el {
			s.push_str("1");
		} else {
			s.push_str("0");
		}
	}
	i32::from_str_radix(&s, 2).unwrap()
}

fn part_one(input: Vec<Vec<bool>>) -> i32 {
	// Find the length of each number - they're all the same
	let num_len = input[0].len();
	let total_rows = input.len();

	// Track number of trues for each position
	let mut true_counts = vec![0; num_len];
	for row in input {
		for (idx, digit) in row.iter().enumerate() {
			if *digit {
				true_counts[idx] += 1;
			}
		}
	}

	// Init gamma
	let mut gamma = vec![false; num_len];
	// Init epsilon
	let mut epsilon = vec![false; num_len];
	// Populate gamma and epsilon
	for (idx, &val) in true_counts.iter().enumerate() {
		if val > total_rows / 2 {
			gamma[idx] = true;
		} else {
			epsilon[idx] = true;
		}
	}

	let numeric_gamma = bools_to_binary(&gamma);
	let numeric_epsilon = bools_to_binary(&epsilon);
	numeric_gamma * numeric_epsilon
}

#[cfg(test)]
mod test {
	use super::*;
	use pretty_assertions::assert_eq;
	#[test]
	fn test_part_one() {
		let input = r#"
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010

"#;

		assert_eq!(part_one(parse_input(input).unwrap()), 198);
	}
}
