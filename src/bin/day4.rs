use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input_day4")?;
    let lines = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let input: Vec<u32> = (&lines[..1]).into_iter().next().unwrap().split(',').map(|number| number.parse::<u32>().unwrap()).collect();
    let mut fastest_bingo = u32::MAX;
    let mut fastest_bingo_score = u32::MAX;

    let mut slowest_bingo = 0;
    let mut slowest_bingo_score = 0;

    let mut bingo_grid: Vec<Vec<(u32,bool)>> = Vec::new();
    for line in &lines[2..]{
        if line == ""{
            let (bingo_draw,score) = evaluate_grid(bingo_grid.clone(), input.clone());
            if bingo_draw < fastest_bingo{
                fastest_bingo = bingo_draw;
                fastest_bingo_score = score;
            }
            if bingo_draw > slowest_bingo{
                slowest_bingo = bingo_draw;
                slowest_bingo_score = score;
            }
            bingo_grid.clear();
        }else{
            let row: Vec<(u32,bool)> = line.split_whitespace().map(|number| (number.parse::<u32>().unwrap(),false)).collect();
            bingo_grid.push(row);
        }
    }
    println!("Fastest Bingo score {}", fastest_bingo_score);
    println!("Fastest Bingo draw {}", fastest_bingo);

    println!("Slowest Bingo score {}", slowest_bingo_score);
    println!("Slowest Bingo draw {}", slowest_bingo);

    Ok(())
}

fn evaluate_grid(mut bingo_grid: Vec<Vec<(u32,bool)>>, draw: Vec<u32>) -> (u32,u32){
    let mut numbers_drawn = 0; 
    for number in draw{
        numbers_drawn += 1;
        for i in 0..5{
            for j in 0..5{
                if bingo_grid[i][j].0 == number{
                    bingo_grid[i][j].1 = true;
                }
            }
        }
        if validate_bingo(bingo_grid.clone()){
            return (numbers_drawn,calc_score(bingo_grid.clone(),number))
        }
    }
    println!("Should never happen");
    (u32::MAX,u32::MAX)
}

fn validate_bingo(bingo_grid: Vec<Vec<(u32,bool)>>) -> bool{
    for i in 0..5{
        let mut i_val = true;
        let mut j_val = true;
        for j in 0..5{
            if !bingo_grid[i][j].1{
                i_val = false;
            }
            if !bingo_grid[j][i].1{
                j_val = false;
            }
        }
        if i_val || j_val {
            return true;
        }
    }
    
    false
}

fn calc_score(bingo_grid: Vec<Vec<(u32,bool)>>, last_number: u32) -> u32{
    let mut sum = 0;
    for i in 0..5{
        for j in 0..5{
            if !bingo_grid[i][j].1 {
                sum +=bingo_grid[i][j].0;
            }
        }
    }
    return last_number*sum;
}