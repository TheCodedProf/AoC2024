#![feature(test)]

extern crate test;

use std::fs;

fn get_char(grid: &Vec<Vec<char>>, (y, x): (usize, usize)) -> char {
	let mut c = '.';
	// println!("{x}, {y}");
	if y < grid.len() && x < grid[y].len()  {
		c = grid[y][x];
	}

	c
}

fn check_surround(grid: &Vec<Vec<char>>, (x, y): (usize, usize)) -> usize {
	let mut n = String::from("X");
	let mut ne = String::from("X");
	let mut e = String::from("X");
	let mut se = String::from("X");
	let mut s = String::from("X");
	let mut sw = String::from("X");
	let mut w = String::from("X");
	let mut nw = String::from("X");


	for diff in 1..4 {
		n.push(get_char(grid, (y.checked_sub(diff).unwrap_or(grid.len()),x))); // y-, x
		ne.push(get_char(grid, (y.checked_sub(diff).unwrap_or(grid.len()), x+diff))); // y-, x+
		e.push(get_char(grid, (y,x+diff))); // y. x+
		se.push(get_char(grid, (y+diff, x+diff))); // y+, x+
		s.push(get_char(grid, (y+diff, x))); // y+, x
		sw.push(get_char(grid, (y+diff, x.checked_sub(diff).unwrap_or(grid[y].len())))); // y+. x-
		w.push(get_char(grid, (y, x.checked_sub(diff).unwrap_or(grid[y].len())))); // y, x-
		nw.push(get_char(grid, (y.checked_sub(diff).unwrap_or(grid.len()), x.checked_sub(diff).unwrap_or(grid[y].len())))); // y-, x+
	}

	// println!("({x},{y}): {n}:{ne}:{e}:{se}:{s}:{sw}:{w}:{nw}");
	let count = [n, ne, e, se, s, sw, w, nw].iter().filter(|s| **s == String::from("XMAS")).count();

	count
}

fn check_middle(grid: &Vec<Vec<char>>, (x, y): (usize, usize)) -> bool {
	let neg_y = y.checked_sub(1).unwrap_or(grid.len());
	let neg_x = x.checked_sub(1).unwrap_or(grid[y].len());

	let nesw = format!("{}A{}", get_char(grid, (neg_y, neg_x)), get_char(grid, (y+1, x+1)));
	let nwse = format!("{}A{}", get_char(grid, (neg_y, x+1)), get_char(grid, (y+1, neg_x)));

	let check = (nesw == "MAS" || nesw.chars().rev().collect::<String>() == "MAS") && (nwse == "MAS" || nwse.chars().rev().collect::<String>() == "MAS");
	// println!("({x},{y}): {nesw} / {nwse} = {check}");

	check
}

fn part_1(input: &Vec<Vec<char>>) -> usize {
	let mut count: usize = 0;

	for (y, line) in input.clone().iter().enumerate() {
		for (x, letter) in line.iter().enumerate() {
			if letter == &'X' {
				count += check_surround(input, (x,y));
			}
		}
	}

	println!("Part 1: {count}");

	count
}

fn part_2(input: &Vec<Vec<char>>) -> usize {
	let mut count: usize = 0;

	for (y, line) in input.clone().iter().enumerate() {
		for (x, letter) in line.iter().enumerate() {
			if letter == &'A' {
				count += if check_middle(input, (x,y)) {1} else {0};
			}
		}
	}

	println!("Part 2: {count}");

	count
}

fn parse_input(input: &str) -> Option<Vec<Vec<char>>> {
	Some(input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>())
}

fn main() {
	let file = fs::read_to_string("./input.txt").expect("Couldn't read file");

	let input = parse_input(file.as_str()).unwrap();
	part_1(&input);
	part_2(&input);
}

#[test]
fn test_main() {
	let file = String::from("MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX");
	let input = parse_input(file.as_str()).unwrap();
	part_1(&input);
	part_2(&input);
}

#[bench]
fn full(b: &mut test::Bencher) {
	b.iter(|| main());
}

#[bench]
fn input_only(b: &mut test::Bencher) {
	let input = fs::read_to_string("./input.txt").expect("Couldn't read file");

	b.iter(|| parse_input(input.as_str()));
}

#[bench]
fn part_one(b: &mut test::Bencher) {
	let file = fs::read_to_string("./input.txt").expect("Couldn't read file");

	let input = parse_input(file.as_str()).unwrap();
	b.iter(|| part_1(&input));
}

#[bench]
fn part_two(b: &mut test::Bencher) {
	let file = fs::read_to_string("./input.txt").expect("Couldn't read file");

	let input = parse_input(file.as_str()).unwrap();
	b.iter(|| part_2(&input));
}
