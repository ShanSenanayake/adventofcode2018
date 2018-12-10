use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;

fn parse_input() -> Vec<i32>  {
    let file = File::open("input/day1_1.txt").expect("file not found");
    let input = BufReader::new(file).lines();
    input
       .map(|line| line.unwrap().parse::<i32>().unwrap())
       .collect::<Vec<i32>>()
}

pub fn one() -> i32 {
    parse_input().into_iter().sum()
}

pub fn two() -> i32 {
    let mut set = HashSet::new();
    let mut acc = 0;
    set.insert(acc);
    parse_input()
        .into_iter()
        .cycle()
        .find_map(|elem| {
            acc += elem;
            set.replace(acc)
        })
        .unwrap()
}