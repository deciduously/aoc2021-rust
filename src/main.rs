use anyhow::Result;
use std::{io::prelude::*, path::PathBuf};

fn get_input(day: u8, part: u8) -> Result<String> {
	let filename = format!("inputs/{}-{}.txt", day, part);
	let mut f = std::fs::File::open(PathBuf::from(filename))?;
	let mut contents = String::new();
	f.read_to_string(&mut contents)?;
	Ok(contents)
}

fn read_nums(input: &str) -> Vec<i32> {
	let mut ret = Vec::new();
	for line in input.lines() {
		let s = line;
		if !s.is_empty() {
			ret.push(s.parse().unwrap());
		}
	}
	ret
}

mod day1 {
	use crate::{get_input, read_nums};
	use anyhow::Result;

	pub fn run() -> Result<()> {
		// both parts use the same input
		let puzzle_input = read_nums(&get_input(1, 1)?);
		println!("part 1: {}", part_one(&puzzle_input));
		println!("part 2: {}", part_two(&puzzle_input));
		Ok(())
	}

	fn part_one(nums: &[i32]) -> i32 {
		let mut largers = 0;
		for (idx, num) in nums.iter().enumerate() {
			if idx > 0 && num > &nums[idx - 1] {
				largers += 1;
			}
		}
		largers
	}

	fn part_two(nums: &[i32]) -> i32 {
		let mut window_sums = Vec::new();
		for idx in 0..nums.len() - 2 {
			window_sums.push(nums[idx] + nums[idx + 1] + nums[idx + 2]);
		}
		part_one(&window_sums)
	}

	#[cfg(test)]
	mod test {
		use super::*;
		use pretty_assertions::assert_eq;
		#[test]
		fn test_part_one() {
			let nums = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
			assert_eq!(part_one(&nums), 7);
		}
		#[test]
		fn test_part_two() {
			let nums = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
			assert_eq!(part_two(&nums), 5);
		}
	}
}

fn main() -> Result<()> {
	let args = std::env::args().collect::<Vec<String>>();
	if args.len() != 2 {
		eprintln!("usage: aoc2021-rust <DAY>");
		std::process::exit(1);
	}
	let day: i32 = args[1].parse().unwrap();
	match day {
		1 => day1::run()?,
		_ => {
			eprintln!("Day not implemented");
			std::process::exit(1);
		}
	}
	Ok(())
}
