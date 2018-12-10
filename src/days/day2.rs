use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

fn parse_input() -> Vec<String>  {
    let file = File::open("input/day2_1.txt").expect("file not found");
    let input = BufReader::new(file).lines();
    input
       .map(|line| line.unwrap())
       .collect::<Vec<String>>()
}

fn count_twos_and_threes(line:String) -> (u32, u32) {
    line
        .chars()
        .fold(HashMap::new(), |mut letters, character| {
            {
                let counter = letters.entry(character).or_insert(0);
                *counter += 1;
            }
            letters
        } )
        .values()
        .fold((0,0), |accumulator, value| {
            let (two, three) = accumulator;
            match value {
                2 => (1, three),
                3 => (two, 1),
                _ => accumulator
            }
        })
}

pub fn one() -> u32 {
    let (two, three) = parse_input()
       .into_iter()
       .map(count_twos_and_threes)
       .fold((0,0), |(two_sum, three_sum), (two, three)| (two+two_sum, three+three_sum));
    two * three
}