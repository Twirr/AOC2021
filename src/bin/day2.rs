use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input_day2")?;
    let lines = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    part1(lines.clone());
    part2(lines.clone());

    Ok(())
}
fn parse_line(line: String) -> (String, u32) {
    let v: Vec<&str> = line.split(" ").collect();
    let direction = v[0].into();
    let steps = v[1].parse::<u32>().unwrap();
    (direction, steps)
}

fn part1(lines: Vec<String>) {
    let mut hori_pos = 0;
    let mut depth = 0;
    for line in lines {
        let (direction, steps) = parse_line(line);

        match direction.as_ref() {
            "down" => depth += steps,
            "up" => depth -= steps,
            "forward" => hori_pos += steps,

            // Handle the rest of cases
            _ => println!("Ain't special"),
        }
    }
    println!("Result1 {}", depth * hori_pos);
}
fn part2(lines: Vec<String>) {
    let mut hori_pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in lines {
        let (direction, steps) = parse_line(line);

        match direction.as_ref() {
            "down" => aim += steps,
            "up" => aim -= steps,
            "forward" => {
                hori_pos += steps;
                depth += aim * steps;
            }

            // Handle the rest of cases
            _ => println!("Ain't special"),
        }
    }
    println!("Result2 {}", depth * hori_pos);
}
