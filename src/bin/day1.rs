use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    println!("Running {}", program);

    let file = File::open("input_day1")?;
    let original_data = BufReader::new(file).lines().map(|line| line.unwrap().parse::<i32>().unwrap()).collect::<Vec<i32>>();

    part1(original_data.clone());

    part2(original_data.clone());
    Ok(())
}
fn part1(original_data: Vec<i32>){
    let mut prev_val = i32::MAX;
    let mut counter = 0;

    for val in original_data.into_iter(){
        if val > prev_val {
            counter += 1;
        }
        prev_val = val;
    }
    println!("Result1: {}", counter);
}

fn part2(original_data: Vec<i32>){
    let mut prev_window = i32::MAX;
    let mut counter = 0;

    for x in 0..original_data.len()-2 {
        let slice = &original_data[x..x+3];
        let window = slice.into_iter().sum::<i32>();
        if window > prev_window {
            counter += 1;
        }
        prev_window = window;
    }
    println!("Result2: {}", counter);
}

