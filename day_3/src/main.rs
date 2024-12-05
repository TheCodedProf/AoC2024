#![feature(test)]

extern crate test;

use regex::Regex;
use std::fs;

enum Value {
	Flip(bool),
	Mult(usize, usize)
}

fn part_1(input: &Vec<Value>) -> usize {
	let sum = input.iter().map(|v| {
		match v {
			Value::Flip(_) => {
				0
			},
			Value::Mult(a, b) => {
				a*b
			},
		}
	}).sum();

	println!("Part 1: {sum}");

	sum
}

fn part_2(input: &Vec<Value>) -> usize {
	let mut enable = true;

	let mut sum: usize = 0;
	for value in input.iter() {
		match value {
			Value::Flip(b) => {enable = *b},
			Value::Mult(a, b) => {
				if enable {
					sum += a*b
				}
			},
		}
	}

	println!("Part 2: {sum}");

	sum
}

fn parse_input(input: &str) -> Option<Vec<Value>> {
	let mul_regex = Regex::new(r"mul\((\d+)\,(\d+)\)|(do\(\))|(don\'t\(\))").unwrap();

	let caps = mul_regex.captures_iter(input);

	Some(
		caps
		.map(|cps: regex::Captures<'_>| {
			let t = cps.get(0).map_or("", |m| m.as_str());
			match t {
				"do()" => Value::Flip(true),
				"don't()" => Value::Flip(false),
				_ => Value::Mult(
					cps.get(1)
					.map_or(0, |m|
						m.as_str().parse::<usize>().unwrap()
					),
					cps.get(2)
						.map_or(0, |m|
							m.as_str().parse::<usize>().unwrap()
						)
					)
			}
		})
		.collect::<Vec<Value>>()
	)
}

fn main() {
	let file = fs::read_to_string("./input.txt").expect("Couldn't read file");

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
