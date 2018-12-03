use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn one() -> i32 {
    let file = File::open("input/day1_1.txt").expect("file not found");

    let input = BufReader::new(file).lines();
    input
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .sum()
}