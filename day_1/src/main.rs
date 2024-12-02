#![feature(test)]

extern crate test;

use std::{collections::HashMap, fs};

fn part_1(left: &Vec<u32>, right: &Vec<u32>) {
    let mut diff: u32 = 0;
	for (i, l) in left.iter().enumerate() {
		diff += l.abs_diff(right[i]);
	}

    println!("Part 1: {diff}");
}

#[derive(Debug, Clone, Copy)]
struct Occurences {
	left: u32,
	right: u32
}

fn part_2(left: &Vec<u32>, right: &Vec<u32>) {
	let mut mapped = HashMap::<u32, Occurences>::new();
	for (_, l) in left.iter().enumerate() {
		let v = &mut mapped.get(l).unwrap_or( &Occurences { left: 0, right: 0 });
		mapped.insert(*l, Occurences { left: v.left + 1, right: v.right });
	}
	for (_, r) in right.iter().enumerate() {
		let v = &mut mapped.get(r).unwrap_or( &Occurences { left: 0, right: 0 });
		mapped.insert(*r, Occurences { left: v.left, right: v.right + 1 });
	}

	let reduce = mapped.iter().map(
		|(num, occurences)| {
			num*occurences.left*occurences.right
		}
	);

	let similarity: u32 = reduce.sum();

	println!("Part 2: {similarity}");
}

fn parse_input() -> Option<(Vec<u32>, Vec<u32>)> {
    let input = fs::read_to_string("./input.txt").expect("Couldn't read file");

	let (mut left, mut right) = input.lines().map(
		|l| {
			let mut split = l.split_whitespace();
			let left = split.next();
			let right = split.next();

			(
				left.unwrap().parse::<u32>().unwrap(),
				right.unwrap().parse::<u32>().unwrap()
			)
		}
	).collect::<(Vec<_>, Vec<_>)>();

	left.sort();
	right.sort();

	Some((left, right))
}

fn main() {
	let (left, right) = parse_input().unwrap();
	part_1(&left, &right);
	part_2(&left, &right);
}

#[bench]
fn full(b: &mut test::Bencher) {
	b.iter(|| main());
}

#[bench]
fn input_only(b: &mut test::Bencher) {
	b.iter(|| parse_input());
}

#[bench]
fn part_one(b: &mut test::Bencher) {
	let (left, right) = parse_input().unwrap();
	b.iter(|| part_1(&left, &right));
}

#[bench]
fn part_two(b: &mut test::Bencher) {
	let (left, right) = parse_input().unwrap();
	b.iter(|| part_2(&left, &right));
}
